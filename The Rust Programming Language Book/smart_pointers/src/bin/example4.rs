// Implicit Deref Coercions with Functions and Methods

use std::ops::Deref;

// Defining Our Own Smart Pointer

struct MyBox<T>(T); // tuple struct with 1 element

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Treating a Type Like a Reference by Implementing the Deref Trait
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
  let m = MyBox::new(Strring::from("Gaurav"));
  hello(&m);
}

