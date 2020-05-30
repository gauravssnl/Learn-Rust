fn main() {
    let string1 = String::from("Rust");
    {
        let string2 = "RustLang";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is: {}", result);
    }
}

// Lifetime Annotations in Function Signatures
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1 // &s1 also works
    } else {
        s2
    }
}
