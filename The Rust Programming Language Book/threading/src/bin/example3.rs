// Using move Closures with Threads

use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    // we need to use move here as closure uses v
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v); // this won't work as value has  been moved into the closure
    handle.join().unwrap(); // handle.join() reeturns a result that must be used, so we call unwrap()
}
