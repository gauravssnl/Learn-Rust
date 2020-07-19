fn main() {
    // The below code won't compile
    // not &str, str on its own are DST or unsized type - size can be known at runtime
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
    // The solution is to use &str ( slice) - contains 2 values : startting position & length
    let s1 = &"Hello there";
    println!("s1 = {}", s1);

    // we need to put values of dynamically sized types behind a pointer of some kind.

    // By default, generic functions will work only on types that have a known size at compile time.
    fn generic<T>(t: T) { // is actually treated as generic<T:Sized>(t: T)
    }
}

// This syntax is only available for Sized, not any other traits.
// T may or may not be Sized
fn generic_taking_unsized_param<T: ?Sized>(t: &T) { // we need to use pointer here as T may or may mot be Sized
}
