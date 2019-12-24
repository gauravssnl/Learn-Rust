// Running Code on Cleanup with the Drop Trait

struct CustomerSmartPointer {
    data: String,
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
   let c = CustomerSmartPointer { data: String::from("my stuff")} ;
   let d = CustomerSmartPointer{ data: String::from("other stuff")};
   println!("CustomerSmartPointer created.");
}