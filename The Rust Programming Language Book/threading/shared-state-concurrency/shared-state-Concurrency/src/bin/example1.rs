use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    println!("m = {:?}", m);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}