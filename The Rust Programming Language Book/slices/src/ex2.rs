fn main() {
    let s = String::from("hello world");
    println!("The first word in string '{}' is '{}'", s, first_word(&s));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    &s
}