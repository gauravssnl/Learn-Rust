// Generic Type Parameters, Trait Bounds, and Lifetimes Together

use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let str1 = "Rust".to_string();
    {
        let str2 = "RustLang".to_string();
        let test = 123;

        let result = longest_with_announcement(&str1, &str2, test);
        println!("Result: {}", result);
    }
}