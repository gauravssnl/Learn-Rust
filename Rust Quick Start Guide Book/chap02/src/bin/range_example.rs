fn main() {
    for x in 1..10 {
        // exclude 10
        print!("{} ", x);
    }
    println!();
    for x in 1..=10 {
        // include 10
        print!("{} ", x);
    }
    println!();
}
