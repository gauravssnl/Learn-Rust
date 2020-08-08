fn main() {
    println!("Hello, world!");
    println!("{} days", 31);

    // Positional arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{:>6}", 1);
    println!("{number:>width$}", number = 1, width = 6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number = 1, width = 6);
    println!("{:>06}", 1);

    // note this will add 15 whitepspace and a "1"
    println!("{:>16}", 1);
}
