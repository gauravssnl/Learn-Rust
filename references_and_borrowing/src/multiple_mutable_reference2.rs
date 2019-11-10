fn main() {
    let mut s = String::from("hello"); 
    {
        let _r1 = &mut s;
    };
    let r2 = &mut s;  // allowed 
    println!("{}", r2);
}

