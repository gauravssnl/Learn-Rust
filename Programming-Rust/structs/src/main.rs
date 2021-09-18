use structs::Queue;
use structs::Node;

fn main() {
   let mut queue = Queue::new();
   queue.push(0);
   queue.push(1);
   println!("{:?}",queue.pop());

   queue.push(2);
   println!("{:?}",queue.pop());
   println!("{:?}",queue.pop());
   println!("{:?}",queue.pop());
   println!("{}", queue.is_empty());

   // Passing Self as Box, Rc or Arc
   let mut bq = Box::new(Queue::new());
   bq.push("Gs Su");
   println!("{:?}", bq);
   println!("{}", bq.is_empty());
   println!("{:?}", bq.pop());
   println!("{}", bq.is_empty());
   println!("{:?}", bq);

   let mut parent = Node::new("parent".to_string());
   use std::rc::Rc;
   let shared_node = Rc::new(Node::new("first".to_string()));
   shared_node.append_to(&mut parent);
   println!("parent node: {:?}", parent);
}
