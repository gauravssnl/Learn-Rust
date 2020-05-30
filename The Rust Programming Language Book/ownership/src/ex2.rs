fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1); // this will not compile as the value has been moved to s2
}
