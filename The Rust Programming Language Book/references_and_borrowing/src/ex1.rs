fn main() {
    let s = String::from("hello");

    change(&s); // this will not compile as the reference is not mutable
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}