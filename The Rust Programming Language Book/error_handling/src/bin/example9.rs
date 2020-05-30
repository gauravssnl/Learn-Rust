use std::fs::File;
use std::io::Error;
use std::io::Read;

// A Shortcut for Propagating Errors: the ? Operator
// Chaning methods

fn read_username_from_file() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    println!("{:?}", read_username_from_file());
}