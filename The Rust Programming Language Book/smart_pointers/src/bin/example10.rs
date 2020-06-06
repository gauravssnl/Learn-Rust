// Creating a Reference Cycle
// Reference Cycles Can Leak Memory

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    // The second element in the Cons variant is now RefCell<Rc<List>>
    // as we want to modify which List value a Cons variant is pointing to.
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use crate::List::{Cons, Nil};

impl List {
    // get the second item of a Cons variant
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(1, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a's next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a  rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b's next item = {:?}", b.tail());

    // modify list in 'a' to point to 'b'
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b's  rc count after changing a  = {}", Rc::strong_count(&b));
    println!("a  rc count after changing a  = {}", Rc::strong_count(&a));
    // below line will overflow the stack
    // println!("a value now: {:?}", a);   // uncomment this line to see cyclic refernce error
    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
