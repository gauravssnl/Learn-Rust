// Default Generic Type Parameters and Operator Overloading
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// the code behind Add trait is trait Add<RHS=Self> , so we need not specify impl Add<RHS=Point> here, even that will work
impl Add for Point {
    // impl Add<Point> for Point also works
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main() {
    assert_eq!(
        Point { x: 1, y: 2 } + Point { x: 4, y: 3 },
        Point { x: 5, y: 5 }
    );
}
