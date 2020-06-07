// Using Message Passing to Transfer Data Between Threads

use std::sync::mpsc; // mpsc stands for mutliple producer single consumer

fn main() {
    // mpsc::channel() returns a tuple - transmitter and receiver
    let (tx, rx) = mpsc::channel(); // pattent matching used for destructuring tuple
                                    // we have to use move here as closeure uses tx transmitter
    std::thread::spawn(move || {
        let value = String::from("Hi Rust");
        tx.send(value).unwrap(); // if error is encounter code will panic beacuse of unwrap()
    });
    let received = rx.recv().unwrap();
    println!("Received message: {}", received);
}
