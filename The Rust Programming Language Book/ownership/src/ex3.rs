fn main() {
    let s1 = String::from("helllo");
    let s2 = s1.clone(); // deeply copy heap data of string
    println!("s1: {}, s2: {}", s1, s2);

    // this workd for integers as they have Copy trait
    // String has Drop trait
    let x = 5;
    let y = x;
    println!("X: {}, y: {}", x, y);
}
