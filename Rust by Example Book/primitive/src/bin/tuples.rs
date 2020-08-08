// Tuples can be used as function arguments and return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // let (integer, boolean) = pair;

    // (boolean, integer)

    // Either use the above code or the below code
    // Values can be extracted from the tuple using tuple indexing
    (pair.1, pair.0)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("{:?}", tuple_of_tuples);

    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (10, true);
    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (12u32,));
    println!("just an integer: {:}", (12u32));

    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{} {} {} {}", a, b, c, d);

    let matrix = Matrix(1.1, 2.2, 3.3, 4.5);
    println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transponse Matrix:\n{}", transponse(matrix));
}

// Activity code
use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", &self.0, &self.1, &self.2, &self.3)
    }
}

/// Return a Matrix in which two elements have been swapped
fn transponse(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}
