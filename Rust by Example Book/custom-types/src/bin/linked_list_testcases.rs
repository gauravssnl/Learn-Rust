enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

use crate::List::*;

// Methods can be attached to an enum
impl List {
    // Create an empty List
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume  a list and return the same list with a new element at its front
    fn prepend(self, element: u32) -> List {
        // `Cons` also has type List
        Cons(element, Box::new(self))
    }

    // Return the length of the List
    fn len(&self) -> u32 {
        // `self` has to be matched, because of the behaviour of this method
        // depends on the variant of the `self'
        // `self` has type `&List` and `*self` has type `List`, matching on a
        // concerete type `T' is preferred over a match on a referece `&T`

        match *self {       //  `match self` also works here
            // can't take ownership of the tail, beacuse `self` is borrowed
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),    //  `Cons(_, tail)` works here if we use `match self` at top
            // Base case : An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match self {                // match &self also works here
            Cons(head, tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
