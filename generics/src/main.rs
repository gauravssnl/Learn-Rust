fn main() {
    let number_list = vec![10, 5, 1, 30, 20, 60, 90, 7, 80];
    println!("The largest number is: {}", largest_i32(&number_list));

    let char_list = vec!['c', 'b', 'Z', 'X', 'z', 'X','\n'];
    println!("The largest character is: {}", largest_char(&char_list));

}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
