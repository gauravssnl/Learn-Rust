use std::fs;

// This program will compile
fn main() ->Result<(), Box< dyn std::error::Error>> {
    let f = fs::File::open("hello.txt")?;
    Ok(())
}