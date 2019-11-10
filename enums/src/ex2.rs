#![allow(unused_variables)]

fn main() {
    println!("Message quit: {:?}", Message::Quit);
    let m = Message::Write(String::from("hello"));
    m.call();
}


#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("The message is {:?}", self);
    }
}