fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

fn main() {
    let string1 = String::from("Rust");
    {
        let string2 = "RustLang";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is: {}", result);
    }
}