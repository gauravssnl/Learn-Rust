// Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    // we have used RefCell to mutate the immutable value internally later
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));
    println!("Cuurent value = {:?}", value);
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));

    println!("current a: {:?}", a);
    println!("current b: {:?}", b);
    println!("current c: {:?}", c);

    // chnage the value stored in Refcell stored inside value variable
    *value.borrow_mut() += 10;    // borrow mutably
    println!("Cuurent value = {:?}", value);
    
    println!("current a: {:?}", a);
    println!("current b: {:?}", b);
    println!("current c: {:?}", c);
}