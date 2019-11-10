fn main() {
    let x = 5; // immutable variable
    println!("The value of x: {}", x);

    // x = 6;  // cant assign value to immutable var

    // shadowing
    let mut x = 1; // mutable variable
    println!("The value of x: {}", x);

    x = 2;
    println!("The value of x: {}", x);

    const PI: f32 = 3.141; // constant has to be declared with type annotation
    println!("The value of PI: {}", PI);

    // Scalar types

    let d = 100_0;
    println!("The value of d: {}", d);

    let h = 0xFF;
    println!("The value of h: {}", h);

    let o = 0o77;
    println!("The value of o: {}", o);

    let b = 0b11011;
    println!("The value of b: {}", b);

    let u = 25u8; // type suffix
    println!("The value of u: {}", u);

    let a = b'A';
    println!("The value of a: {}", a);

    let f: f32 = 240.09;
    println!("The value of f: {}", f);

    let heart_eyed_cat = '😻';
    println!("Heart Eyed Cat: {}", heart_eyed_cat);

    let c = 'C';
    println!("The value of c: {}", c);

    let t: bool = true;
    println!("The value of t: {}", t);

    // Compound Types

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is {}, y is  {}, z is {}", x, y, z);
    // Accessing values by index
    println!("The value at index 1 in tuple tup is {}", tup.0);

    // Arrays
    let arr = [1, 2, 3, 4];
    println!("The value of arr is: {:#?}", arr); // pretty-print

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of arr is: {:?}", arr);
    // acess element
    println!("The value of arr[2] is: {}", arr[2]);

    // array with same value for 10 times
    let arr = [5; 10];
    println!("The value of arr is: {:?}", arr);
}
