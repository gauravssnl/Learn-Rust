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
    println!("{:?}", Student::new("Rust".to_string(), 1));
}

#[derive(Debug)]
struct Student {
    id: u128,
    name: String,
    subjects: Vec<String>,
    marks_list: Vec<u32>,
}

impl Default for Student {
    fn default() -> Self {
        Student {
            id: 0,
            name: String::default(),
            subjects: vec![],
            marks_list: vec![],
        }
    }
}

impl Student {
    fn new(name: String, id: u128) -> Self {
        Student {
            name,
            id,
            ..Default::default() // use default values
        }
    }
}
