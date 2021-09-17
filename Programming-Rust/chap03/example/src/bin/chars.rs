fn main() {
    println!("{}", '*'.is_alphabetic());
    println!("{:?}", '8'.to_digit(10));
    println!("{}", 'ಠ'.len_utf8());
    println!("{:?}", std::char::from_digit(2, 10));
}