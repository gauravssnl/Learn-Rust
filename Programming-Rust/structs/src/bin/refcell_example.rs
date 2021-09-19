use std::cell::RefCell;

fn main() {
    let ref_cell = RefCell::new("hello".to_string());
    let mut r = ref_cell.borrow_mut();
    r.push_str(" world");
    println!("ref_cell: {:?}", &ref_cell);
}