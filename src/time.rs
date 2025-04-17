use std::{error::Error, fmt::Display};

use chrono::{DateTime, Datelike, Local, NaiveDateTime, ParseError, TimeZone, Timelike, Utc};
pub const DEFAULT_FORMAT: &str = "%Y/%m/%d %H:%M";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeStamp {
    unix_secs: i64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn year(&self) -> i32 {
        self.fields.year()
    }
    pub fn month(&self) -> u8 {
        self.fields.month().try_into().unwrap()
    }
    pub fn day(&self) -> u8 {
        self.fields.day().try_into().unwrap()
    }
    pub fn hour(&self) -> u8 {
        self.fields.hour().try_into().unwrap()
    }
    pub fn minute(&self) -> u8 {
        self.fields.minute().try_into().unwrap()
    }
}

impl From<i64> for TimeStamp {
    fn from(value: i64) -> Self {
        Self { unix_secs: value }
    }
}
impl From<&i64> for TimeStamp {
    fn from(value: &i64) -> Self {
        Self::from(*value)
    }
}

impl From<&TimeStamp> for i64 {
    fn from(value: &TimeStamp) -> Self {
        value.unix_secs
    }
}
impl From<TimeStamp> for i64 {
    fn from(value: TimeStamp) -> Self {
        Self::from(&value)
    }
}

impl From<&TimeFields> for TimeStamp {
    fn from(value: &TimeFields) -> Self {
        Self {
            unix_secs: value.fields.timestamp(),
        }
    }
}
impl From<TimeFields> for TimeStamp {
    fn from(value: TimeFields) -> Self {
        Self::from(&value)
    }
}

impl From<&TimeStamp> for TimeFields {
    fn from(value: &TimeStamp) -> Self {
        Self {
            fields: DateTime::from_timestamp(value.unix_secs, 0).unwrap(),
        }
    }
}
impl From<TimeStamp> for TimeFields {
    fn from(value: TimeStamp) -> Self {
        Self::from(&value)
    }
}

impl Display for TimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for TimeParseError {}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_FORMAT, TimeFields, TimeStamp};

    #[test]
    fn time_fields() {
        let timestamp = TimeStamp::from(1_743_044_280);
        let fields = TimeFields::new(2025, 3, 27, 2, 58).unwrap();
        assert_eq!(fields, TimeFields::from(timestamp));
    }

    #[test]
    fn time_now() {
        let time1 = TimeStamp::now();
        let time2 = TimeStamp::now();
        assert!(time1 <= time2);
    }

    #[test]
    fn time_str_conv() {
        let str = "2025/03/27 02:58";
        let time = TimeStamp::from(TimeFields::new(2025, 3, 27, 2, 58).unwrap());

        let formatted = time.format_str_fmt(DEFAULT_FORMAT);
        assert_eq!(str, formatted);

        let parsed = TimeStamp::parse_str_fmt(str, DEFAULT_FORMAT).unwrap();
        assert_eq!(time, parsed);
    }
}
