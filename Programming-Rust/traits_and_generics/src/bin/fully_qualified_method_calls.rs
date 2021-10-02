fn main() {
    let s1 = "hello".to_string();

    let s2 = str::to_string("hello"); // qualified method call

    let s3 = <str as ToString>::to_string("hello"); // fully qualified method call

    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);

    // when the type of self can't be inferred.
    let _zero = 0;
    // let _r = _zero.abs(); // error does not work
    let _r = i64::abs(_zero);

    // when using function itself as function value
    let lines = r"Rust is cool.
Python is cool.
Golang is cool.
Is Java cool ?";
    let words: Vec<String> = lines.split_whitespace().map(ToString::to_string).collect();
    println!("words: {:?}", words);
}
