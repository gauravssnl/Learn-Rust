#![allow(unused_variables)]

fn main() {
    // the static lifetime
    // the lifetime of all string literals is 'static.
    let s: &'static str = "I have a static lifetime";
    test1(s);
    test2(s);
}

fn test1(s: &str) {
    println!("String: {}", s);
}

fn test2(s: &str) {
    println!("String: {}x", s);
}