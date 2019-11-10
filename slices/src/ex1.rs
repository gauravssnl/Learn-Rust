fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("{}", hello);
    println!("{}", &s[..5]);
    println!("{}", &s[..]);
    println!("{}", &s); 
}