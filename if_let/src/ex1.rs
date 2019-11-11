// need of if let sample
fn main() {
    // match an optional value but only if it is 3
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => ()
    }
}