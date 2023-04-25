use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    MissingInputFilePath,
    MissingOutputFilePath,
    IOError(std::io::Error),
    LZFSEError(lzfse_rust::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingInputFilePath => {
                write!(f, "missing input file path")
            }
            Error::MissingOutputFilePath => {
                write!(f, "missing output file path")
            }
            Error::IOError(io_error) => {
                write!(f, "io error: {}", io_error)
            }
            Error::LZFSEError(lzfse_error) => {
                write!(f, "lzfse error: {}", lzfse_error)
            }
        }
    }
}
impl error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value)
    }
}
impl From<lzfse_rust::Error> for Error {
    fn from(value: lzfse_rust::Error) -> Self {
        Error::LZFSEError(value)
    }
}
