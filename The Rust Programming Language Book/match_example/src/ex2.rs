fn main() {
    
    fn plus_one(x: Option<i32>) -> Option<i32> {
        // Matches are exhaustive
        match x {
            Some(i) => Some(i+1),
            None => None
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    println!("Six: {:?}", six);
    let none = None;
    println!("{:?}", plus_one(none));
}