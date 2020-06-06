use std::rc::Rc;

// Note Rc<T> is only suitable for single-threaded code
// Rc<T> allows you to share data between multiple parts of your program for reading only.
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// Generic Cons List with Rc 
enum GenericList<T> {
    Cons(T, Rc<GenericList<T>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a : {:?}", a);
    println!("Count after creating a = {}", Rc::strong_count(&a));
    // we can also use a.clone(), but the convention is to use Rc::clone
    // Rc::clone doesn't make deep copy
    let b = Cons(3, Rc::clone(&a));    // if we use Box here, value a will be moved and we can't us it later
    println!("b : {:?}", b);
    println!("Count after creating b = {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("c : {:?}", c);
    println!("Count after creating c = {}", Rc::strong_count(&a));
    {
        let d = Cons(4, Rc::clone(&a));
        println!("d : {:?}", d);
        println!("Count after creating d = {}", Rc::strong_count(&a));
    }
    println!("Count after  d goes out of scope = {}", Rc::strong_count(&a));
}
