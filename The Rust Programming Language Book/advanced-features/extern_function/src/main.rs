// Using extern Functions to Call External Code - calling them  are unsafe
// keyword extern facilitates the creation and use of a Foreign Function Interface (FFI).
// demonstrates how to set up an integration with the abs function from the C standard library
// The "C" part defines which application binary interface (ABI) the external function uses: 
//  the ABI defines how to call the function at the assembly level. 
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(3));
    }
}

// // Calling Rust Functions from Other Languages
// // need to add a #[no_mangle] annotation to tell the Rust compiler not to mangle the name of this function
// #[no_mangle]
// pub extern "C" fn call_from_c() {
//     println!("Just called a Rust function from C!");
// }