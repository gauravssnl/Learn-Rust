use std::sync::Arc; // atomic reference counted type
use std::sync::Mutex;
use std::thread;

fn main() {
    // Mutex<T> provides interior mutability, as the Cell family does.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // shadow count variable & use clone to have multiple owners of count variable
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for  handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.lock().unwrap());
}
