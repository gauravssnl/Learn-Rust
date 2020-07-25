use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std:: thread;


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// a type alias for a trait object that holds the type of closure that execute receives. 
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new Threadpool
    /// 
    /// The size is the number of the threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The 'new' function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create some threads and store in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /* We still use the () after FnOnce because this FnOnce represents a closure 
    that takes no parameters and returns the unit type (). Just like function definitions, 
    the return type can be omitted from the signature, but even if we have no parameters, we still need the parentheses. */

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static, {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}

pub struct Worker {
    id: usize,
    thread: std::thread::JoinHandle<()>,                 // JoinHandle T type will be unit type ()
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Acquiring a lock might fail if the mutex is in a poisoned state, 
            // which can happen if some other thread panicked while holding the lock rather than releasing the lock.
            // so we call unwrap() after acquiring lock
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing", id);
            job();
           
        });

         // The below code doesn’t result in the desired threading behavior
        // let thread = thread::spawn(move || {
        //     let job = receiver.lock().unwrap().recv().unwrap();
        //     while let Ok(job) =  receiver.lock().unwrap().recv() {
        //         println!("Worker {} got a job; executing", id);
        //         job();
        //     }
        // });

        Worker {
            id,
            thread,              // a thread spawned with an empty closure. or use thread::spawn(|| {}),
        }
    }
}