pub struct Constrained {
    pub min: i32,
    pub max: i32,
    current: i32,
}

impl Constrained {
    pub fn set(&mut self, value: i32) {
        self.current = value;
    }

    pub fn get(&self) -> i32 {
        if self.current < self.min {
            self.min
        } else if self.current > self.max {
            self.max
        } else {
            self.current
        }
    }

    pub fn can_fail(x: bool) -> Result<i32, &'static str> {
        if x {
            Ok(5)
        } else {
            Err("x is false")
        }
    }
    pub fn can_fail_without_return_value<'a>(x: bool) -> Result<(), &'a str> {
        if x {
            Ok(()) // contains the success value
        } else {
            Err("x is false")
        }
    }
}
fn main() {
    let c = Constrained {
        min: 10,
        max: 20,
        current: 40,
    };
    println!("{}", c.get());
    println!("Ok");
}
