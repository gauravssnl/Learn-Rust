#[allow(unused)]

fn main() {
    // Variables can be type annotated.
    let flag: bool = true;

    let a_float = 2.0; // Regular annotation
    let an_integer = 5u32;

    // or a default will be used.
    let default_float = 3.0; //  `f64`
    let default_integer = 7; // `i32`

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 1234567890i64;

    // A mutable's variable value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 15;

    // Error! The type of the variable can't be changed.
    // Uncomment the next line to see the error
    // mutable = true;

    // Variables can be overwritten with shadowing.a_float
    let mutable = true;
}
