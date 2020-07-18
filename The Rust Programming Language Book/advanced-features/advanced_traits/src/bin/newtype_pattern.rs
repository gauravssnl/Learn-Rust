// Using the Newtype Pattern to Implement External Traits on External Types
// The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for

use std::fmt;

// the wrapper type will not have methods of Vector by default
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

// we will implment Dref trait to make Wrapper a Smart Pointer; we are doing this to use Vec methods on Wrapper
impl std::ops::Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let wrapper = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("wrapper = {}", wrapper);
    // we will use Vec<String> len() method directly as it is a smart pointer
    println!(
        "Length of vector embdeded within wrapper : {}",
        wrapper.len()
    );
    // Even indexing works
    println!("First element: {}", wrapper[0]);
}
