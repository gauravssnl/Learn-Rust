pub trait Mul<RHS = Self> {
    type Output;

    fn mul(self, rhs: RHS) -> Self::Output;
}

struct WindowSize {
    data: i32,
}

// Implement foreign trait for foreign type , using a type defined here

impl<T> Mul<WindowSize> for T {
    type Output = f64;
    fn mul(self, rhs: WindowSize) -> Self::Output {
        2 as Self::Output * rhs.data as Self::Output // just a dummy example
    }
}

fn main() {}
