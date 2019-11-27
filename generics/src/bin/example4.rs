struct Point<T> {
    x: T,
    y: T,
}

// method for generic Point<T>
impl <T> Point<T> {
    fn  x(&self) -> &T {
        &self.x
    }
}

// method for Point<i32> 
impl Point<i32> {
   fn test(self) -> String {
       "test Point<i32>".to_string()
   }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let point1 = Point{x: 5, y: 6};
    println!("point1.x: {}", point1.x());
    // Point<i32>
    let point1 = Point{x: 5i32, y: 6i32};
    println!("point1.x: {}", point1.test());

    let point1 = Point{x: 4.0, y: 3.0};
    println!("Distance of p1 from origin: {}", point1.distance_from_origin());
}