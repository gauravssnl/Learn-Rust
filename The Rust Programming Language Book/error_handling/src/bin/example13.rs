// Creating Custom Types for Validation
use std::fmt::Display;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32,) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess{
            value
        }
    }

    // used for reading value
    pub fn value(self) -> i32 {
        self.value
    }
}



fn main() {
    let guess1 = Guess::new(20);
    let guess2 = Guess::new(101);
}