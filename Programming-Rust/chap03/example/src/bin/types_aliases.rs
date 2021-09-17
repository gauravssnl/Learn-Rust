type Bytes = Vec<u8>;

fn main() {
    let mut bytes = Bytes::new();
    for b in b"Hello" {
        bytes.push(*b);
    }
    println!("{:?}", bytes);
}