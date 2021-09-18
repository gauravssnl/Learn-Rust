use error_handling::{self, GenericError};
use std::io;

fn main() {
    let io_error = io::Error::new(io::ErrorKind::Other, "timed out");
    // let err : Result<GenericError<>> = Err(GenericError::from(io_error));

    // let io_error = io::Error::new(io::ErrorKind::Other, "timed out");
    error_handling::print_error(&io_error);
}
