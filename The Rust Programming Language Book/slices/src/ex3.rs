fn main() {
    let s = String::from("hello world");
    println!("The first word in string '{}' is '{}'", s, first_word(&s));
    s.clear(); // error because we have already passed mutable reference to first_word
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