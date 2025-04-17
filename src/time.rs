use chrono::{DateTime, Datelike, Local, TimeZone, Timelike, Utc};
pub const DEFAULT_FORMAT: &str = "%Y/%m/%d %H:%M";

#[derive(Debug)]
pub struct TimeStamp {
    unix_secs: i64,
}

#[derive(Debug)]
pub struct TimeFields {
    fields: DateTime<Utc>,
}

impl TimeStamp {
    pub fn now() -> Self {
        Self {
            unix_secs: Local::now().naive_local().and_utc().timestamp(),
        }
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

impl From<TimeFields> for TimeStamp {
    fn from(value: TimeFields) -> Self {
        Self {
            unix_secs: value.fields.timestamp(),
        }
    }
}

impl From<TimeStamp> for TimeFields {
    fn from(value: TimeStamp) -> Self {
        Self {
            fields: DateTime::from_timestamp(value.unix_secs, 0).unwrap(),
        }
    }
}
