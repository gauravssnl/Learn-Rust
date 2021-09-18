use std::error::Error;
use std::io::{self, Write};

/// Dump an error message to `stderr`.
///
/// If another error happens while building the error message or
/// writing to `stderr`, it is ignored.
pub fn print_error(mut error: &dyn Error) {
    let _ = writeln!(io::stderr(), "error: {}", error);
    while let Some(source) = error.source() {
        let _ = writeln!(io::stderr(), "caused by {}", source);
        error = source;
    }
}

use std::fs;
use std::path::Path;

pub fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?
    }
    Ok(())
}

use std::io::BufRead;

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GenericResult<T> = Result<T, GenericError>;

pub fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}
