// tell the Rust compiler not to mangle the name of this function
#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust to C");
}