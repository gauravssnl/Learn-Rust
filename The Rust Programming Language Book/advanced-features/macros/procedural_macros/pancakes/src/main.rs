use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Panckes;

fn main() {
    Panckes::hello_macro();
}
