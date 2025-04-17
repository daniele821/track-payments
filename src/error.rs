use std::{error, result};

#[derive(Debug)]
pub enum Error {
    GenericError(Box<dyn error::Error>),
}

pub type Result<T> = result::Result<T, Error>;

impl From<Box<dyn error::Error>> for Error {
    fn from(value: Box<dyn error::Error>) -> Self {
        Self::GenericError(value)
    }
}
