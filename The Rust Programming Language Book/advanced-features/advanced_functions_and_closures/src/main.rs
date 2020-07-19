// Function Pointers

fn add_one(x: i32) -> i32 {
    x + 1
}

// Using the fn type to accept a function pointer as an argument
// Unlike closures, fn is a type rather than a trait,
// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), 
// so you can always pass a function pointer as an argument for a function that expects a closure.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
fn main() {
    let answer = do_twice(add_one, 5);
    println!("answer = {}", answer);
}
