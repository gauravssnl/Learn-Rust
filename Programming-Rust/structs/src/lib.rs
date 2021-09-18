/// A first-in, first-out queue queue implementation.
#[derive(Debug)]
pub struct Queue<T> {
    older: Vec<T>,  // older elements, eldest last
    younger: Vec<T> // younger elements,youngest last
}

impl<T> Queue<T> {
    /// Push a chatacter onto the back of a queue.
    pub fn push(&mut self, data: T) {
        self.younger.push(data);
    }
    
    /// Pop the data off the front of the queue. Return `Some(T)` if there 
    /// was a character to pop, or `None` if the queue was empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // Bring the elements in younger over to older, and put them in 
            // the promised order.
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);

            // Now we need to reverse the older vec to bring eldest element at the end.__rust_force_expr!
            self.older.reverse();
        }
        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }

    pub fn new() -> Self {
        Queue { younger: vec![], older: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.younger.is_empty() && self.older.is_empty()
    }
}

use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    tag: String,
    children: Vec<Rc<Node>>,
}

impl Node {
    pub fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }

    pub fn new(tag: String) -> Self {
        Node { tag: tag, children: vec![]}
    }
}

// Associated Consts
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2{ x:0.0, y:0.0 };
    const UNIT: Vector2 = Vector2{ x:1.0, y: 0.0 };
    const NAME: &'static str = "Vector2";
    const ID:u32 = 18;
}