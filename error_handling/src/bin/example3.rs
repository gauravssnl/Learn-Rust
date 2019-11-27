use std::fs::File;
use std::io::ErrorKind;

// matching on different errors

fn main() {
    let f = File::open("hello.txt");
    
    match f {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(error) => panic!("Problem creating the file: {:?}", error),
                },
                other_error =>  panic!("Problem opeing the file: {:?}", error),
            }
        },
    };
}