mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
    mod serving {
            fn take_order() {}
            fn serve_order() {}
            fn take_payment() {}
        }
        pub mod test {
            pub fn testing() {}
        }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

// Calling a function using a relative path starting with super

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}

    // Making Structs and Enums Public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: String) -> Breakfast {
            Breakfast {
                toast,
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    pub enum Appetizer {
        Salad,
        Soup,
    }
}

pub fn eat_at_new_restaurant() {
    let mut meal = back_of_house::Breakfast::summer(String::from("Rye"));
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.",  meal.toast);
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}

// brinng a module into scope 
// Idiomatic use Paths ( not impoting function directly)
// bringing in structs, enums, and other items with use
use crate::front_of_house::hosting;

pub fn eat_at_new_restaurant_1() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// using relative path
// Providing New Names with the as Keyword
use self::front_of_house::hosting as hosting_new;

pub fn eat_at_new_restaurant_2() {
    hosting_new::add_to_waitlist();
    hosting_new::add_to_waitlist();
    hosting_new::add_to_waitlist();
}

use std::collections::HashMap;
fn main() {
   let mut map = HashMap::new();
   map.insert(1,2);
   println!("map: {:?}", map);
}

// Bringing two types with the same name into the same scope requires using their parent modules.
use std::io;
use std::fmt;

// fn function1() -> io::Result {
    
// }

// fn function2() -> fmt::Result {
//     return std::fmt::Result::Err;
// }

// Re-exporting Names with pub use

pub use front_of_house::test;

pub fn test_function1() {
    test::testing();
}

// Using Nested Paths to Clean Up Large use Lists
/* Normal approach 
use std::io;
use std::cmp::Ordering;
*/
 // use std::{cmp::Ordering, io};

 // Two use statements where one is a subpath of the other 
 /* Normar approcah 
 use std::io;
 use std::io::Write;
 */
 // use std::io{self, Write};

 // The Glob operator - import everything into scope
 // not used generally
 // The glob operator is often used when testing to bring everything under test into the tests module;
 // sometimes used as part of the prelude pattern
 // use std::collections::*;