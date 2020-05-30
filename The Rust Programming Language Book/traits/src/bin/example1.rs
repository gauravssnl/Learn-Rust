use std::fmt::Display;

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { // here Pair<T> works
        Self { // // here Pair<T> does not work
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is x: {}", self.x);
        } else {
            println!("The largest number is y: {}", self.y);
        }
    }
}

/* We can also conditionally implement a trait for any type that implements another trait. 
Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations
impl<T:Display> ToString for T {
    // sample code not exact working code
    fn to_string(&self) -> String {
        return String::from(self);
    }
}

*/


fn main() {
    let pair = Pair::new(1, 2);
    println!("pair: {:?}", pair);
    pair.cmp_display();
}