fn largest<T: PartialOrd >(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    &largest
}

fn main() {
    let number_list = vec![10, 1, 20, 70, 40, 30, 90, 50, 34, 56];
    println!("The largest element in number_list: {}", largest(&number_list));

    let char_list = vec!['C', 'a', 'X', 'A', 'x', 'c', 'R', 'P'];
    println!("The largest element in char_list: {}", largest(&char_list));

    let str_list = vec!["Python", "Rust", "Go", "rust"];
    println!("The largest element in str_list: {}", largest(&str_list));

    let string_list = vec!["Python".to_string(), "Rust".to_string(), "Go".to_string(), "rust".to_string()];
    println!("The largest element in str_list: {}", largest(&string_list));
}
