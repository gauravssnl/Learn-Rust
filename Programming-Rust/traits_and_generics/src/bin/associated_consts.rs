trait Greet {
    const GREETING : &'static str = "Hello";
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
}