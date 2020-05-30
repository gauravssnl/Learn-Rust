use std::fs;

// This program won't compile
fn main() {
    fs::File::open("hello.txt")?;
}