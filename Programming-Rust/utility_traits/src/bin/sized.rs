// Marker Trait - Sized
struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T,
}

use std::fmt::Display;

fn main() {
    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "Rust".to_string(),
    };

    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;

    display(&boxed_displayable);
}

// Conversion happens implicitly when passing values to functions
fn display(boxed: &RcBox<dyn Display>) {
    println!("Boxed value: {}", &boxed.value);
}
