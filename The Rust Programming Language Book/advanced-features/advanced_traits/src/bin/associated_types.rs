struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

// trait Iterator uses associated type Item

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        while self.count < 10 {
            self.count += 1;
            return Some(self.count);
        }
        None
    }
}

struct GenericCounter<T> {
    count: T,
}

impl<T> GenericCounter<T> {
    fn new(start_count: T) -> GenericCounter<T> {
        GenericCounter { count: start_count }
    }
}

impl Iterator for GenericCounter<i64> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.count < 10 {
            self.count += 1;
            return Some(self.count);
        }
        None
    }
}

// Trait using generics
pub trait Foo<T> {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Foo<f32> for GenericCounter<f32> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0.0 {
            self.count -= 1.0;
            return Some(self.count as Self::Item);
        }
        None
    }
}

// next method will become ambiguous for GenericCounter<f64> as trait Iterator & Foo<f64> defines next()
impl Iterator for GenericCounter<f64> {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.count < 10.0 {
            self.count += 1.0;
            return Some(self.count as Self::Item);
        }
        None
    }
}

impl Foo<f64> for GenericCounter<f64> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0.0 {
            self.count -= 1.0;
            return Some(self.count as Self::Item);
        }
        None
    }
}

fn main() {
    let counter = Counter::new();
    for val in counter {
        println!("{}", val);
    }

    // here we have to declare counter as mut as we call next method
    let mut counter = Counter::new();
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("Next Val: {:?}", counter.next());
    println!("{}", "*".repeat(40));

    let mut generic_counter = GenericCounter::new(0i64);
    println!("Next Val: {:?}", generic_counter.next());
    println!("Next Val: {:?}", generic_counter.next());
    println!("Next Val: {:?}", generic_counter.next());
    println!("Next Val: {:?}", generic_counter.next());
    println!("{}", "*".repeat(40));

    let mut generic_counter = GenericCounter::new(10f32);
    println!("Next Val: {:?}", generic_counter.next());
    println!("Next Val: {:?}", generic_counter.next());
    println!("Next Val: {:?}", generic_counter.next());
    println!("Next Val: {:?}", generic_counter.next());
    println!("{}", "*".repeat(40));

    let mut generic_counter = GenericCounter::new(5f64);
    // we need to disambiguate next() method here as both Foo trait & Iterator trait is implemnted for GenericCounter<f64>
    println!(
        "Next Val: {:?}",
        <GenericCounter<f64> as Foo<f64>>::next(&mut generic_counter)
    );
    println!("Next Val: {:?}", Foo::next(&mut generic_counter));
    println!("Next Val: {:?}", Foo::next(&mut generic_counter));
    println!("Next Val: {:?}", Foo::next(&mut generic_counter));
    println!("Next Val: {:?}", Foo::next(&mut generic_counter));
    println!("Next Val: {:?}", Foo::next(&mut generic_counter));
    println!("{}", "*".repeat(40));

    let mut generic_counter = GenericCounter::new(5f64);
    println!("Call GenericCounter<f64> Iterator next method");
    // we need to disambiguate next() method here as both Foo trait & Iterator trait is implemnted for GenericCounter<f64>
    println!(
        "Next Val: {:?}",
        <GenericCounter<f64> as Iterator>::next(&mut generic_counter)
    );
    println!(
        "Next Val: {:?}",
        std::iter::Iterator::next(&mut generic_counter)
    );
    println!("Next Val: {:?}", Iterator::next(&mut generic_counter));
    println!("Next Val: {:?}", Iterator::next(&mut generic_counter));
    println!("Next Val: {:?}", Iterator::next(&mut generic_counter));
    println!("Next Val: {:?}", Iterator::next(&mut generic_counter));
    println!("Next Val: {:?}", Iterator::next(&mut generic_counter));
    println!("{}", "*".repeat(40));
}
