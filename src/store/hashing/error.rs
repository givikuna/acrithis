use std::{fmt, io};

#[derive(Debug)]
pub enum HashingError {
    Io(io::Error),
    InvalidLength(usize),
    InvalidHex,
}

impl fmt::Display for HashingError {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            | HashingError::Io(err) => write!(f, "I/O error during hashing: {}", err),
            | HashingError::InvalidLength(length) => {
                write!(f,
                       "Invalid hash length: expected 64, got {}",
                       length)
            },
            | HashingError::InvalidHex => write!(f, "Hash contains invalid non-hex characters"),
        }
    }
}

impl std::error::Error for HashingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            | HashingError::Io(err) => Some(err),
            | _ => None,
        }
    }
}
