use std::iter;
use std::vec::IntoIter;

fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// Needs heap allocation
fn cyclical_zip1(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item = u8>> {
    Box::new(v.into_iter().chain(u.into_iter()).cycle())
}

// Better version than v2 - Erase the type of the return value
// we can change the actual type being returned in future as long as it implements the same trait
fn cyclical_zip3(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}

use std::fmt::Display;

fn print<T: Display>(val: T) {
    println!("{}", val);
}

fn display(val: impl Display) {
    println!("{}", val);
}

fn main() {
    print::<i32>(5);
    print("String".to_string());
    display(5);
    // explicit generic argument not allowed
    // display::<String>("String".to_string());
}
