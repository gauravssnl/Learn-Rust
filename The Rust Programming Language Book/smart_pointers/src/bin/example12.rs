// Visualizing Changes to strong_count and weak_count
// Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>
// Creating a Tree Data Structure: a Node with Child Nodes
// Rc<T> instance is only cleaned up if its strong_count is 0
// irrespective of weak_count

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    // RefCell is used here beacuse we might change the children if required
    // we want Node to own childern that's why we have used Rc<Node>
    children: RefCell<Vec<Rc<Node>>>,
    // Adding a Reference from a Child to Its Parent - we should use Weak here
    // beacuse if childern Node goes out of scope, parent Node shouldn't go away
    // if we use Rc<T>, it will lead to cyclic reference from parent to child & child to parent
    // added to go to branch from leaf node
    // A node will be able to refer to its parent node but doesn’t own its parent
    parent: RefCell<Weak<Node>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 10,
        children: RefCell::new(vec![]), // as this is a leaf node(last node) , it has no children,
        parent: RefCell::new(Weak::new()), // initially set node parent empty, we will modify later
    });

    println!("leaf: {:?}", leaf);
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf intiial rc strong count: {}, weak count: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
    {
        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });
        println!("branch: {:?}", branch);
        println!("leaf current rc strong count: {}", Rc::strong_count(&leaf));
        println!(
            "branch current rc strong count: {}",
            Rc::strong_count(&branch)
        );
        println!("branch current rc weak count: {}", Rc::weak_count(&branch));

        // now set parent Node of leaf to branch Node
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // Rc::downgrade gives weak reference to Rc<T> , i.e. Weak<T>
        println!("leaf now: {:?}", leaf);
        // brach strong count will remain same
        println!(
            "branch current rc strong count: {}",
            Rc::strong_count(&branch)
        );
        println!("branch current rc weak count: {}", Rc::weak_count(&branch));
    } // branch dropped here - strong_countof  Rc<Node> becomes 0

    // borrow immutably
    // upgrade returns Option<Rc<T>>
    //  leaf.parent will be None now as branch Node strong_count is 0 & has been dropped
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf intiial rc strong count: {}, weak count: {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)   // the weak count will be 0 now as variable leaf is the only reference 
    );
}
