use thiserror::Error;

#[derive(Debug, Error)]
#[error("{message:} ({line:}, {column})")]
pub struct JsonError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}



fn main() {
    let err = JsonError {
        message: "expected ']' at the end of array".to_string(),
        line: 1,
        column: 25,
    };
    println!("{}", err);
}