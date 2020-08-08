use std::fmt;

// Define a structure named 'List' containing a 'Vec'
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to 'Vec'
        let vec = &self.0;

        write!(f, "[")?;

        for (index, value) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator try! to return on errors.
            if index != 0 {
                write!(f, ", ")?;
            }

            // write the value
            write!(f, "{}", value)?;

            // write the index & value
            // write!(f, "{}: {}", index, value)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let list = List(vec![1, 2, 3, 5, 6]);
    println!("{}", list);
}
