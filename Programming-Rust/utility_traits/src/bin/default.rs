trait Default {
    fn default() -> Self;
}

#[derive(Debug)]
struct List<T> {
    data: Vec<T>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List { data: vec![] }
    }
}

fn main() {
    let list: List<String> = List::default();
    println!("{:?}", list);

    use std::collections::HashSet;
    let squares = [4, 9, 16, 25, 36, 49, 64, 81];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
        squares.iter().partition(|&n| n & n - 1 == 0);
    println!("powers_of_two: {:?}, impure: {:?}", powers_of_two, impure);

    let (upper, lower): (String, String) = "Great Teacher Onizuka"
        .chars()
        .partition(|&c| c.is_uppercase());
    println!("upper: {:?}, lower: {:?}", upper, lower);
}
