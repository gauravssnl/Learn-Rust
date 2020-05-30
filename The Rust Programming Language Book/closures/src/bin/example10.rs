// Capturing the Environment with Closures

fn main() {
    let x = vec![1, 2, 3, 4];

    let equal_to_x = move |z| z == x;  // closure takes ownership of the vector value

    println!("The value of x: {:?}", x); // this line will throw error
    let y = vec![1, 2, 3, 4];

    assert!(equal_to_x(y));
}