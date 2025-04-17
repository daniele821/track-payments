use crate::time::TimeParseError;
use std::{error, fmt::Display};

#[derive(Debug)]
pub enum Error {
    TimeParseError(TimeParseError),
    GenericError(Box<dyn error::Error>),
}

impl From<TimeParseError> for Error {
    fn from(value: TimeParseError) -> Self {
        Self::TimeParseError(value)
    }
}

impl From<Box<dyn error::Error>> for Error {
    fn from(value: Box<dyn error::Error>) -> Self {
        Self::GenericError(value)
    }
}
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::GenericError(Box::from(value))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TimeParseError(err) => write!(f, "{err}"),
            Error::GenericError(err) => write!(f, "{err}"),
        }
    }
}
impl std::error::Error for Error {}
