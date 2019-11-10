fn main() {
    let some_number = Some(5);
    println!("some number: {:?}", some_number);
    let some_string = Some("hello");
    println!("some_string: {:?}", some_string);
    let absent_number: Option<i32> = None;
    println!("absent_number: {:?}", absent_number);
    // unwraps original value
    println!("Number {}", some_number.unwrap());
    println!("String {}", some_string.unwrap());
    // println!("Absent number: {}", absent_number.unwrap()); // cause panick
}
