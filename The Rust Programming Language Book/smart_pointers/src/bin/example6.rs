// Dropping a Value Early with std::mem::drop

struct CustomerSmartPointer {
    data: String,
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomerSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomerSmartPointer c  created.");
    // c.drop() destructor exists, but that explicit call isn't allowed.
    std::mem::drop(c);
    let d = CustomerSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomerSmartPointer d created.");
}
