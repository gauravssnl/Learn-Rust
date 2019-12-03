#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            true
        } else {
            false
        }
    }
}

pub fn add_two(x: i32) -> i32 {
    x+ 2 
    // x + 3
}

pub fn greeting(name: &str) -> String {
   format!("Hello, {}", name)
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100");
        }
        Self {
            value,
        }
    } 
}

use std::fmt::Display;

fn prints_and_returns_value<T>(t: &T) -> &T
    where T: Display 
{
    println!("Input value: {}", t);
    t
}

pub fn add_one(a: i32) -> i32 {
    internal_adder(a, 1)
}

// private function 
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {width: 20, height: 10};
        let smaller = Rectangle {width: 10, height: 9};

        assert!(larger.can_hold(smaller)); // asert checks for bool true as result
    }

    #[test]
    fn smaller_cannot_hold_larger() {
         let larger = Rectangle {width: 20, height: 10};
        let smaller = Rectangle {width: 10, height: 9};

        assert!(!smaller.can_hold(larger));   
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2)); // assert_eq tests for equal value and display them if they are not equal

    }

    #[test]
    fn three_plus_two_test() {
        assert_ne!(-1, add_two(3)); // test for not equal value 
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Gaurav");
        assert!(result.contains("Gaurav"));
    }

     #[test]
     #[ignore]
    fn greeting_contains_name_fail_test() {
        let result = greeting("Gaurav");
        assert!(
            result.contains("Rust"),
            "Greeting does not contain name '{}', the value was '{}'", "Rust", result);
    }

    #[test]
    #[should_panic]
    fn greater_than_hundred_guess() {
        Guess::new(101);
    }

    // should_panic is generally used with expected message
    // panic message should contain that expected message
    #[test]
    #[should_panic(expected="Guess value must be between 1 and 100")]
    fn less_than_one_guess_correct_panic_message_test() {
        Guess::new(0);
    }

    #[test]
    #[ignore]
    #[should_panic(expected="Guess value must be between 0 and 100")]
    fn less_than_one_guess_correct_panic_message_fail_test() {
        Guess::new(0);
    }

    // Using Result<T, E> in Tests
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn test_prints_and_returnd_value() {
        assert_eq!(10, *prints_and_returns_value(&10));
    }

    // Ignoring Some Tests Unless Specifically Requested
    // to run ignored test : cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_test() {
        // expensive function test
    }

    // test private function
    #[test]
    fn test_internal_addrer() {
        assert_eq!(5, internal_adder(3, 2));
    }
  }
