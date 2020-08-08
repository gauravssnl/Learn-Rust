#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let p1 = Point(1, 2);
    println!("p1: {:?}", p1);
    println!("{:#?}", p1);

    let person = Person {
        name: "Gs",
        age: 34,
    };
    println!("{:?}", person);
    println!("{:#?}", person);
}
