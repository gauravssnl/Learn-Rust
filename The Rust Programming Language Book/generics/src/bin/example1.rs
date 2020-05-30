#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 1, y: 2 };
    println!("integer_point: {:?}", integer_point);
    let float_point = Point { x: 1.0, y: 3.0 };
    println!("float_point: {:?}", float_point);
}
