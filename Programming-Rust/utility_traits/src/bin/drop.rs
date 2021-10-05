struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!();
    }
}
fn main() {
    let mut a = Appellation {
        name: "Zeus".to_string(),
        nicknames: vec![
            "cloud collector".to_string(),
            "king of the gods".to_string(),
        ],
    };

    println!("Before assignment");
    a = Appellation {
        name: "Hera".to_string(),
        nicknames: vec![],
    };
    println!("at the end of the block");
}

// standard prelude drop function example
fn drop<T>(_x: T) {}

use libc;

struct FileDesc {
    fd: i32, // actual type : c_int
}

impl Drop for FileDesc {
    fn drop(&mut self) {
        let _ = unsafe {
            libc::close(self.fd);
        };
    }
}