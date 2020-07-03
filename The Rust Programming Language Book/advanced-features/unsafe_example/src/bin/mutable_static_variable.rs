// static variable example - use Screaming SnakeCase
// acts as global variables
static HELLO_WORLD: &str = "Hello, world!";
// mutable static variable
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // modifying mutable static variable is unsafe
    unsafe { COUNTER += inc }
}

fn main() {
    println!("{}", HELLO_WORLD);
    add_to_count(3);
    // Accessing mutable static variable is unsafe
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
