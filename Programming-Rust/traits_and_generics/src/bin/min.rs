fn min<T>(value1: T, value2: T) -> T
where
    T: PartialOrd,
{
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

fn main() {
    println!("{}", min(1, 2));
    println!("{}", min(0.1, 0.09));
    // ASCII value comparisions
    println!("{}", min("a", "b"));
    println!("{}", min("A", "a"));
    println!("{}", min("B", "a"));
}
