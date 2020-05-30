#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl <T, U> Point<T, U> {
    fn  mixin<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let point1 = Point{x: 1.0f32, y: 5i32};
    let point2 = Point{x: 3.20f64, y: 1};
    println!("Mix point1 & point2: {:?}", point1.mixin(point2));

    let point1 = Point{x: 1.0f32, y: 5i32};
    let point3 = Point{x: "Hello", y: 'R'};
    println!("Mix point1 & point2: {:?}", point1.mixin(point3));
}

