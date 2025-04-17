use crate::time::TimeError;
use std::error;

#[derive(Debug)]
pub enum Error {
    TimeError(TimeError),
    GenericError(Box<dyn error::Error>),
}

impl From<TimeError> for Error {
    fn from(value: TimeError) -> Self {
        Self::TimeError(value)
    }
}

impl From<Box<dyn error::Error>> for Error {
    fn from(value: Box<dyn error::Error>) -> Self {
        Self::GenericError(value)
    }
}
