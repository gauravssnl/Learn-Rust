fn main() {
    let s = String::from("hello world");
    println!("The first word in string '{}' is '{}'", s, first_word(&s));
    let s1 = "hello world";
     println!("The first word in string '{}' is '{}'", s, first_word(&s1));
     // this also works as string literals are string slices already
     println!("The first word in string '{}' is '{}'", s, first_word(s1));
}

fn first_word(s: &str) -> &str { // &str can aceept parameter of &String also
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    &s
}