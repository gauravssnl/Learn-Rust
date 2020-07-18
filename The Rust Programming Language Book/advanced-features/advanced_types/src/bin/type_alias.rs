// Creating Type Synonyms with Type Aliases

fn main() {
    type KiloMeters = f32;
    let distance: KiloMeters = 63.5;
    println!("distance = {}", distance);
    // The main use case for type synonyms is to reduce repetition. For example, we might have a lengthy type like this:
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(_f: Box<dyn Fn() + Send + 'static>) {}

    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        Box::new(|| ())
    }

    // Thunk is a word for code to be evaluated at a later time
    type Thunk = Box<dyn Fn() + Send + 'static>;

    fn takes_long_type_new(_f: Thunk) {}

    fn returns_long_type_new() -> Thunk {
        Box::new(|| ())
    }

    // Type aliases are also commonly used with the Result<T, E> type for reducing repetition
    use std::fmt;
    use std::io::Error;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<(), Error>;

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        fn write_fmt(&mut self, f: fmt::Arguments) -> Result<(), Error>;
    }
}
