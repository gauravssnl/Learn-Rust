trait Super {
    fn m1() {}
}

trait Sub: Super {
    fn m2() {}
}

struct Demo;

impl Super for Demo {}

impl Sub for Demo {}

trait SuperTrait {}

// Another way of declaring the sub trait as bound

trait SubTrait
where
    Self: SuperTrait,
{
}

fn main() {}
