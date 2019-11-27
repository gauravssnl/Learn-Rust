fn main() {
    let mut s = String::new();
    s.push_str("Hello");
    println!("s: {}", s);
    s = "Hello, World!".to_string();
    println!("s: {}", s);
    s = String::from("Hello, world");
    println!("s: {}", s);
    let hello = String::from("नमस्ते"); // size 18 bytes
    println!("hello: {}", hello);
    println!("hello length: {}", hello.len());
    s = "LO".to_string();
    s.push('L');
    println!("s: {}", s);
    // concatenation with + operator
    let mut s1 = "Hello";
    let mut s2 = "World";
    let mut s3 = s1.to_owned() + &s2; // take ownership of string s1
    s3 = "Hello, ".to_string() + &"World";
    println!("s3: {}", s3);
    let s1 = "Rust".to_string();
    let s2 = "is".to_string();
    let s3 = "cool".to_string();
    println!("{}", format!("{} {} {}", s1, s2, s3));

    // Slicing string
    let hello = "नमस्ते";
    println!("{}", &hello[..3]);

    // methods of iterating over strings
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("b: {}", b);
    }
}

