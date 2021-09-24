use std::io::Write;

fn say_hello<T>(mut out: T) -> std::io::Result<()>
where
    T: Write,
{
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

    let v2 = (0..100).collect::<Vec<i32>>();
    println!("v2: {:?}", v2);

    let sink = std::io::sink();
    say_hello(sink);
    
}


// Type aliases can be dummy too
type IoResult<T> = Result<T, std::io::Error>;