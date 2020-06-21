// Refutability: Whether a Pattern Might Fail to Match

fn main() {
    let some_option_value: Option<i32> = None;
    // let accepts only irrefutable paatern
    // let Some(x) = some_option_value;    //   uncomment this line to see the error
    // we need to use pattern here by using if let
    if let Some(x) = some_option_value {
        println!("x: {}", x);
    }
    // irrefutable if-let pattern
    if let x = 5 {
        println!("x: {}", x);
    }
}
