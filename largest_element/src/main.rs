fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![10, 1, 20, 70, 40, 30, 90, 50, 34, 56];
    println!("The largest element in number_list: {}", largest(&number_list));

    let char_list = vec!['C', 'a', 'X', 'A', 'x', 'c', 'R', 'P'];
    println!("The largest element in char_list: {}", largest(&char_list));
}
