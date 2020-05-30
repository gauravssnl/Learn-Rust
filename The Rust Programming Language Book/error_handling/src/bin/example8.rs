use std::fs::File;
use std::io::Error;
use std::io::Read;

// A Shortcut for Propagating Errors: the ? Operator

fn read_username_from_file() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    println!("{:?}", read_username_from_file());
}