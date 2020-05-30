fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s; 
    println!("{}, {}", r1, r2 );
    // r1 and r2 are no longer in use after here

    let r3 = &mut s;  //  no problem
    println!("{}", r3);
}

