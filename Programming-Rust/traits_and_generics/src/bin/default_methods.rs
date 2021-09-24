use std::io::Write;

struct Sink;

type IoResult<T> = std::io::Result<T>;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}

fn main() {
    let mut data = b"Hello, Rust";
    let num_bytes = Sink.write(data).unwrap();
    println!("data: {:?}", data);
    println!("{}, {}", num_bytes, data.len());
}
