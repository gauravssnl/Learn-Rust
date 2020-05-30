fn main() {
    let rect = Rectangle{
        length: 10,
        breadth: 20
    };
    println!("The rectangle rect is {:?}", rect); // {:?} is used for debugging 
    println!("The are of rect is {}", area(&rect));
}

// used for debugging 
#[derive(Debug)]
struct Rectangle {
    length : u32,
    breadth: u32
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.breadth
}