fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // not allowed 
    println!("{}, {}", r1, r2);
}

