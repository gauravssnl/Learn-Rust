// Creating Multiple Producers by Cloning the Transmitter

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    // let us cone transmitter tx as we are going to use it in different threads;
    // tx is actually mpsc::Sender instance
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let values = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // use rx as iteratot to read all messages
    for received in rx {
        println!("Got: {}", received);
    }
}
