fn main() {
    let mut s = String::from("hello");

    change(&mut s); // this will  compile as the reference is  mutable
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
