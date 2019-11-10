fn main() {
    let rect = Rectangle {
        length: 10,
        breadth: 20,
    };
    println!("The rectangle rect is {:?}", rect); // {:?} is used for debugging
    println!("The are of rect is {}", area(&rect));
    println!("The are of rect is {}", rect.area());

    let rect2 = Rectangle {
        length: 9,
        breadth: 10,
    };
    println!("Rectangle rect can hold rect2 : {}", rect.can_hold(&rect2));
    let square = Rectangle::square(10);
    println!("the square is {:?}", square);
}

// used for debugging
#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32,
}

// methods
// multiple impl blocks are allowed
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }
    // methods with more parameters
    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.length > other_rectangle.length && self.breadth > other_rectangle.breadth
    }

    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            breadth: size,
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.breadth
}
