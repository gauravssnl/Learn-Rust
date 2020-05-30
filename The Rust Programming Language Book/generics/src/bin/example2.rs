#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer_point = Point { x: 1, y: 2 };
    println!("integer_point: {:?}", integer_point);
    let float_point = Point { x: 1.0, y: 3.0 };
    println!("float_point: {:?}", float_point);
    let mixed_point1 = Point { x: 1, y: 2.0 };
    println!("mixed_point1: {:?}", mixed_point1);
    let mixed_point2 = Point { x: 1.0, y: 2 };
    println!("mixed_point2: {:?}", mixed_point2);
    let mixed_point3 = Point { x: 1.0, y: 2.0 };
    println!("mixed_point3: {:?}", mixed_point3);
    let mixed_point4 = Point { x: 1, y: 2 };
    println!("mixed_point4: {:?}", mixed_point4);
}
