use std::sync::Mutex;
use std::sync::thread;

fn main() {
    let counter = Mutex::mew(0);
    let mut handles = vec![];
}
