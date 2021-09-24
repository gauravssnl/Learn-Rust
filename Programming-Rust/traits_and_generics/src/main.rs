use std::io::Write;

// This function accepts trait objects which implements Write trait
fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"Hello, Rust")?;
    out.flush()
}

fn main() {
    use std::fs::File;
    let mut local_file = File::create("hello.txt").unwrap();
    say_hello(&mut local_file).unwrap();

    let mut bytes = vec![];
    say_hello(&mut bytes).unwrap();
    println!("{:?}", bytes);
    assert_eq!(bytes, b"Hello, Rust");
}
