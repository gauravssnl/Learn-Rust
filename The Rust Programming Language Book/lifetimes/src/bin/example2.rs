fn main() {
    let string1 = String::from("Rust");

    let string2 = "RustLang";

    let result = longest(string1, string2);
    println!("The longest string is: {}", result);
}

fn longest(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() {
        &s1
    } else {
        &s2
    }
}