// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human; // unit struct without data

impl Pilot for Human {
    fn fly(&self) {
        println!("Captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Let's get started");
    }
}

impl Human {
    fn fly(&self) {
        println!("Waving arms furiously.");
    }
}

fn main() {
    let person = Human;
    person.fly();

    Pilot::fly(&person);

    Wizard::fly(&person);
}