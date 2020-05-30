mod lib;

use crate::lib::NewsArticle;
use lib::Tweet;
// import trait
use lib::Summary;


fn main() {
    let article = NewsArticle {
        headline: String::from("Rust is the best language"),
        location: String::from("IN"),
        author: String::from("gauravssnl"),
        content: String::from("Borrow checker and expresssion, Traits, and Structs"),
    };

    println!("Summary of article: {}", article.summarize());
    println!("Summary of article: {}", article.default_summarize());

    let tweet = Tweet {
        username: "gauravssnl".to_string(),
        content: "Rust is cool".to_string(),
        reply: false,
        retweet: false,
    };
    println!("Summary of tweet: {}", tweet.summarize());
    println!("Summary of tweet: {}", tweet.default_summarize());

    // Traits as parameters
    pub fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    notify(article);
    notify(tweet);

    // Trait bound syntax

    pub fn notify1<T: Summary>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }
    // value was moved in earlier function notify()
    // we need to re-initialize again
    let article = NewsArticle {
        headline: String::from("Rust is the best language"),
        location: String::from("IN"),
        author: String::from("gauravssnl"),
        content: String::from("Borrow checker and expresssion, Traits, and Structs"),
    };

    let tweet = Tweet {
        username: "gauravssnl".to_string(),
        content: "Rust is cool".to_string(),
        reply: false,
        retweet: false,
    };

    notify(article);
    notify(tweet);

    /*
    The impl Trait syntax is convenient and makes for more concise code in simple cases.
    pub fn notify(item1: impl Summary, item2: impl Summary) { }

    If we wanted to force both parameters to have the same type,
    that’s only possible to express using a trait bound, like this:
    pub fn notify<T: Summary>(item1: T, item2: T) { }

    Specifying Multiple Trait Bounds with the + Syntax
    pub fn notify(item: impl Summary + Display) { }
    // trait bound syntax
    pub fn notify<T: Summary + Display>(item: T) {
    */

    // Clearer Trait Bounds with where Clauses
    // example:  fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {}

    use core::fmt::Debug;
    use std::fmt::{Display};
    fn some_function<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        return 1;
    }

    // Returning Types that Implement Traits
    // we can return only 1 type
    fn return_summarizable() -> impl Summary {
        Tweet {
            username: "gauarv".to_string(),
            content: "Test tweet".to_string(),
            reply: true,
            retweet: true,
        }
    }

    let tweet = return_summarizable();
    println!("Summary of tweet: {}", tweet.summarize());
}
