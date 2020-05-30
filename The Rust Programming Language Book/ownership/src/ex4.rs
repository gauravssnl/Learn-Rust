// ownership and functions

fn main() {
    let s = String::from("hello"); // s comes into scope

    take_ownership(s); // s value has been moved into the function ...
                       // ... and so is no longer valid here

    //println!("s: {}", s);

    let x = 5; // x comes into scope
    make_copy(x); // x would move into the functoion,
                  // but i32 is Copy, so it's still okay
                  // to still use x afterwards
    println!("x: {}", x);
} // here x goes out of scope, then s. but because s's value was moved,
  // nothing special happens

fn take_ownership(_some_string: String) { // underscore used to suppress warning of unused variable
} // here _some_string goes out of scope and `drop` is called. The backing
  // memory is freed

fn make_copy(_some_integer: i32) {} // here _some_integer goes out of scope, Nothing specails happens
