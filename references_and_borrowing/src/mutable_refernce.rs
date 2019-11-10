fn main() {
    let mut s = String::from("hello");

    change(&mut s); // this will not compile as the reference is not mutable
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
