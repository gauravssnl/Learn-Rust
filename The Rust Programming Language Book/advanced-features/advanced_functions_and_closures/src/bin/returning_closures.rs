// we can't return closures from a fuction directly as they don;t have a size known at compile time
// uncomment the below lines to see the error
// fn returns_closure1() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

// The solution is to use Trait Object
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    // let closure_box = returns_closure();
    println!("{}", returns_closure()(3));
}
