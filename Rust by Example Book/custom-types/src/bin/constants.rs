// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access the constant
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access the constant in the main thread.
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Can't modify a `const`
    // THRESHOLD = 5;           // Uncomment this line to see the error

}
