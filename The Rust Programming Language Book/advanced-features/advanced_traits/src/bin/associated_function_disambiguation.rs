// A trait with an associated function and a type with an associated function of the same name that also implements the trait

trait Animal {
    fn baby_name() -> String;
}

struct Dog; // An unit struct without data

impl Dog {
    fn baby_name() -> String {
        String::from("Tom")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called as {}", Dog::baby_name());
    // The below code won't compile
    // println!("A baby dog is called as {}", Animal::baby_name());
    // use the implementation of Animal for Dog
    // In general, fully qualified syntax is defined as follows:
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called as {}", <Dog as Animal>::baby_name());
}
