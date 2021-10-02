trait Greet {
    const GREETING: &'static str = "Hello";
    fn greet(&self) -> String;
}

trait Float {
    // Associated const does not require value in the declaration.
    const ZERO: Self;
    const ONE: Self;
}

impl Float for f32 {
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
}

impl Float for f64 {
    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
}

struct DummyStr;

impl Greet for DummyStr {
    fn greet(&self) -> String {
        <Self as Greet>::GREETING.to_string()
    }
}

fn main() {
    println!("f32 ZERO = {}, ONE = {}", f32::ZERO, f64::ONE);
    println!("f642 ZERO = {}, ONE = {}", f64::ZERO, f64::ONE);
    println!("Greet trait's GREET: {}", DummyStr::GREETING);
    println!("{}", add_one(2.5));
    println!("{}", fib::<f64>(10));
}

use std::ops::Add;

fn add_one<T: Float + Add<Output = T>>(value: T) -> T {
    value + T::ONE
}

fn fib<T: Float + Add<Output = T>>(n: usize) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n - 1) + fib::<T>(n - 2),
    }
}
