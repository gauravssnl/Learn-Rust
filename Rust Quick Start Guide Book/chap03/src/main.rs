// Ownership & Lifetime examples

fn main() {
    let s1 = "Rust".to_string();
    receive_ownership(s1);
    // println!("{}", s1); // uncomment this line to see the error
    let s2 = String::from("Python");
    let s2 = receive_ownership_and_return_ownership(s2);
    println!("{}", s2);
    let s3 = "Rust";
    borrow_ownership(s3);
    println!("{}", s3);
    let mut x = 2;
    println!("x now : {}", x);
    borrow_ownership_mutably(&mut x);
    println!("x now : {}", x);
    set_to_siz(&mut x);
    println!("x now : {}", x);
}

fn receive_ownership(s: String) {
    println!("{}", s);
}

fn receive_ownership_and_return_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn borrow_ownership(s: &str) {
    println!("Hello, {}", s);
}

fn borrow_ownership_mutably(x: &mut i32) {
    *x = *x + 1;
}

fn set_to_siz(value: &mut i32) {
    *value = 6;
}

pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    // lifetime ellision
    pub fn smaller_x<'a>(value1: &'a Point2D, value2: &'a Point2D) -> &'a f64 {
        if value1.x < value2.x {
            &value1.x
        } else {
            &value2.x
        }
    }

    pub fn smaller_y<'a>(value1: &'a Point2D, value2: &'a Point2D) -> &'a f64 {
        if value1.y < value2.y {
            &value1.y
        } else {
            &value2.y
        }
    }

    // Moving self
    // we can use this in builder pattern orn when we want to invalidate the previous state
    #[allow(dead_code)]
    fn transpose(self) -> Point2D {
        Point2D {
            x: self.y,
            y: self.x,
        }
    }
    #[allow(dead_code)]
    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
