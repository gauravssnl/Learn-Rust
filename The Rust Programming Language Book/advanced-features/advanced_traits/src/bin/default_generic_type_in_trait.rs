// Default Generic Type Parameters and Operator Overloading
use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);

struct Meters(u32);

// we specify RHS ( Right Hand Side Type ) for + operator here afer impl Add
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + (other.0 * 1000)) // convert Meters to Millimeters
    }
}

fn main() {
    let millimeters_val = Millimeters(1200);
    let meters_val = Meters(2);
    println!("{:?}", millimeters_val + meters_val);
}
