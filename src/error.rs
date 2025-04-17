use crate::time::TimeParseError;
use std::error;

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
