use std::fmt;

fn main() {
    type Result<T> = std::result::Result<T, std::io::Error>;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;

        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, f: fmt::Arguments) -> Result<()>;
    }
}