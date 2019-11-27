use std::fs;
use std::io::Error;
use std::io::Read;


fn read_username_from_file() -> Result<String, Error> {
   // direct method to file as string 
   fs::read_to_string("hello.txt")
}

fn main() {
    println!("{:?}", read_username_from_file());
}