// just a small example how we can use function pointers

fn setup(mesage: String) {
    println!("Setting up system");
    println!("Your message: {}", mesage);
}

fn log(f: fn(String), message: String) {
    println!("Calling the given function f :{:?}", f); // this will print function pointer address
    setup(message);
    println!("The call to given function ends here");
}

fn main() {
    log(setup, String::from("Hello, Rust"));
}