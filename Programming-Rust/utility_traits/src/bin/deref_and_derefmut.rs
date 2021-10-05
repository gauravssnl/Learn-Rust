trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
}

trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}

#[derive(Debug)]
struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> std::ops::Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

impl<T> std::ops::DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

fn main() {
    let mut s = Selector {
        elements: vec![
            "Erlang".to_string(),
            "Python".to_string(),
            "Rust".to_string(),
        ],
        current: 2,
    };
    println!("Value at current index in s: {:?}", *s);
    // Apply String methods directly on Selector<String> - Deref Corecion
    println!("{}", s.to_lowercase());

    *s = "Rustlang".to_string();
    println!("Value at current index in s: {:?}", *s);

    show_it(&s); // Deref Corecion : &selector<String> -> &String -> &str
    use std::ops::Deref;
    show_it(s.deref());
    show_it_generic(&*s);
    show_it_generic(&(*s));
    show_it_generic(&s as &str);
}

fn show_it(thing: &str) {
    println!("{}", thing);
}

use std::fmt::Display;
fn show_it_generic<T: Display>(thing: T) {
    println!("{}", thing);
}
