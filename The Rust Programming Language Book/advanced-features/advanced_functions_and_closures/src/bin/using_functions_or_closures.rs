// an example of where you could use either a closure defined inline or a named function

fn main() {
    let list_of_numbers = vec![1, 2, 3, 4];
    // Using closure as an argument to map()
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("{:?}", list_of_strings);

    // using function as an argument to map()
    let list_of_numbers = vec![5; 10]; // it will make a Vec containing 5 10 times
    let list_of_strings = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>(); // turbofish syntax
    println!("{:?}", list_of_strings);
}
