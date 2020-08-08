use std::fmt;

struct Number(i32);

impl fmt::Display for Number {
    // This trait requires 'fmt' method with exact signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let number = Number(12);
    println!("number = {}", number);
}