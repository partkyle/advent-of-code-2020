use core::num::ParseIntError;
use std::convert::From;
use std::io;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error { msg: e.to_string() }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error { msg: e.to_string() }
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error { msg: e }
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error { msg: e.to_string() }
    }
}
