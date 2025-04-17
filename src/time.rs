use chrono::{DateTime, Datelike, Local, ParseError, Timelike, Utc};
use std::{error::Error, fmt::Display};
pub const DEFAULT_FORMAT: &str = "%Y/%m/%d %H:%M";

#[derive(Debug)]
pub struct TimeStamp {
    unix_secs: i64,
}

#[derive(Debug)]
pub struct TimeFields {
    fields: DateTime<Utc>,
}

#[derive(Debug)]
pub enum TimeError {
    ParseError(ParseError),
    InvalidFields(TimeFields),
}

impl TimeStamp {
    pub fn now() -> Self {
        Self {
            unix_secs: Local::now().naive_local().and_utc().timestamp(),
        }
    }
}

impl TimeFields {
    pub fn years(&self) -> i32 {
        self.fields.year()
    }
    pub fn months(&self) -> u8 {
        self.fields.month().try_into().unwrap()
    }
    pub fn days(&self) -> u8 {
        self.fields.day().try_into().unwrap()
    }
    pub fn hours(&self) -> u8 {
        self.fields.hour().try_into().unwrap()
    }
    pub fn minutes(&self) -> u8 {
        self.fields.minute().try_into().unwrap()
    }
}

impl From<i64> for TimeStamp {
    fn from(value: i64) -> Self {
        Self { unix_secs: value }
    }
}

impl From<TimeFields> for TimeStamp {
    fn from(value: TimeFields) -> Self {
        Self {
            unix_secs: value.fields.timestamp(),
        }
    }
}

impl Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for TimeError {}
