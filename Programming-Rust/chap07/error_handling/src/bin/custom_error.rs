use std::fmt;

#[derive(Debug, Clone)]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

// Errors should implement the std::error::Error trait,
// but the default definitions for the Error methods are fine.
impl std::error::Error for JsonError {}

fn main() {
    println!("{:?}", make_sample_json_error());
}

fn make_sample_json_error() -> Result<(), JsonError> {
    Err(JsonError {
        message: "expected ']' at the end of array".to_string(),
        line: 1,
        column: 25,
    })
}
