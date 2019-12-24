// Following the Pointer to the Value with the Dereference Operator

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}