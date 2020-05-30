// Creating Our Own Iterators with the Iterator Trait

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter{ count: 0 }
    }

    //  fn iter(&mut self) -> Option<u32> {
    //     Counter::next(mut self)
    // }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            return Some(self.count);
        } else {
            None
        }
    }
   

}

fn main() {
    let mut counter = Counter::new();
    println!("Got: {:?}", counter.next());
    println!("Got: {:?}", counter.next());
    println!("Got: {:?}", counter.next());
    println!("Got: {:?}", counter.next());
    println!("Got: {:?}", counter.next());
    println!("Got: {:?}", counter.next());
    println!("Got: {:?}", counter.next());

}

#[test]

fn using_other_trait_methods() {
    let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x|  x % 3 == 0)
    .sum();
    assert_eq!(18, sum);
}