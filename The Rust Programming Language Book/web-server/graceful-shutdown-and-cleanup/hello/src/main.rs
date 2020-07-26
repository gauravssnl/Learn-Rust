fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    // we have limited incoming request to 2 to demo graceful shutwdown and cleanup is working in order
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        println!("{}", "*".repeat(80));
        println!("New Client");
        println!("Stream: {:?}", stream);
        // Single-threaded code
        // handle_connection(stream);

        // Multi-threaded code
        // The below line will keep on creating threads for each new connection
        // std::thread::spawn(|| {handle_connection(stream)});
        // so we will use ThreadPool
        // We’ll limit the number of threads in the pool to a small number to protect us from Denial of Service
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 512];
    std::io::prelude::Read::read(&mut stream, &mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\r\n", "404.html")
    };

    let contents = std::fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    println!("Sending response to the client: {:?}", stream.peer_addr());
    std::io::prelude::Write::write(&mut stream, response.as_bytes()).unwrap();
    std::io::prelude::Write::flush(&mut stream).unwrap();
    println!("{}", "*".repeat(80));
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = std::sync::mpsc::channel();

        let receiver = std::sync::Arc::new(std::sync::Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some ThreadPool Workers and store them in the vector
            workers.push(Worker::new(id, std::sync::Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<Message>>>,
    ) -> Worker {
        // we need the closure to loop forever, asking the receiving end of the channel for a job
        // and running the job when it gets one.
        let thread = std::thread::spawn(move || loop {
            // we need to use  move as we are using id inside closure
            let job = receiver.lock().unwrap().recv().unwrap();
            match job {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing it", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
                    break; // break out of the loop
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// a type alias for a trait object that holds the type of closure that execute receives
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                println!("Shutting down worker {}", worker.id);
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}
