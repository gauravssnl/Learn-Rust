fn main() {
    let string1 = String::from("RustLang");
     let result;    

    {
        let string2 = String::from("Rust");

        result = longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is: {}", result);
}

// Lifetime Annotations in Function Signatures
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1 // &s1 also works
    } else {
        s2
    }
}
