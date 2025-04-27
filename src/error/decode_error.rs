use std::error::Error;
use std::{fmt, io};
use std::fmt::{Display, Formatter};
use miniz_oxide::inflate::DecompressError;


#[derive(Debug)]
pub enum DecodeError {
    IOError(std::io::Error),
    SignatureError,
    ChecksumError,
    DecompressError(DecompressError),
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            DecodeError::IOError(ref err) => write!(f, "IO error: {}", err),
            DecodeError::SignatureError => write!(f, "Signature error"),
            DecodeError::ChecksumError => write!(f, "Checksum error"),
            DecodeError::DecompressError(ref err) => write!(f, "Decompression error: {}", err),
        }
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DecodeError::IOError(ref e) => Some(e),
            DecodeError::SignatureError => None,
            DecodeError::ChecksumError => None,
            DecodeError::DecompressError(_) => Some(self),
        }
    }
}

impl From<std::io::Error> for DecodeError {
    fn from(err: std::io::Error) -> DecodeError {
        DecodeError::IOError(err)
    }

}

impl From<DecompressError> for DecodeError {
    fn from(err: DecompressError) -> DecodeError {
        DecodeError::DecompressError(err)
    }
}
