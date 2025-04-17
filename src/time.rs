use std::{error::Error, fmt::Display};

use chrono::{DateTime, Datelike, Local, NaiveDateTime, ParseError, TimeZone, Timelike, Utc};
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
pub struct TimeParseError(ParseError);

impl TimeStamp {
    pub fn now() -> Self {
        Self {
            unix_secs: Local::now().naive_local().and_utc().timestamp(),
        }
    }

    pub fn parse_str_fmt(str: &str, format: &str) -> Result<Self, TimeParseError> {
        NaiveDateTime::parse_from_str(str, format)
            .map(|time| Self::from(time.and_utc().timestamp()))
            .map_err(TimeParseError)
    }
    pub fn parse_str(str: &str) -> Result<Self, TimeParseError> {
        Self::parse_str_fmt(str, DEFAULT_FORMAT)
    }

    pub fn format_str_fmt(&self, format: &str) -> String {
        TimeFields::from(self).fields.format(format).to_string()
    }
    pub fn format_str(&self) -> String {
        self.format_str_fmt(DEFAULT_FORMAT)
    }
}

impl TimeFields {
    pub fn new(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> Option<Self> {
        Utc.with_ymd_and_hms(year, month, day, hour, minute, 0)
            .single()
            .map(|fields| Self { fields })
    }

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

impl From<&TimeFields> for TimeStamp {
    fn from(value: &TimeFields) -> Self {
        Self {
            unix_secs: value.fields.timestamp(),
        }
    }
}

impl From<&TimeStamp> for TimeFields {
    fn from(value: &TimeStamp) -> Self {
        Self {
            fields: DateTime::from_timestamp(value.unix_secs, 0).unwrap(),
        }
    }
}

impl Display for TimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for TimeParseError {}
