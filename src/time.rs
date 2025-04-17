use chrono::Local;

pub const DEFAULT_FORMAT: &str = "%Y/%m/%d %H:%M";

#[derive(Debug)]
pub struct DateTime {
    unix_secs: i64,
}

#[derive(Debug)]
pub struct DateTimeFields {
    years: i32,
    months: u8,
    days: u8,
    hours: u8,
    minutes: u8,
}

impl DateTime {
    pub fn now() -> Self {
        Self {
            unix_secs: Local::now().naive_local().and_utc().timestamp(),
        }
    }
}

impl DateTimeFields {
    pub fn new(years: i32, months: u8, days: u8, hours: u8, minutes: u8) -> Self {
        Self {
            years,
            months,
            days,
            hours,
            minutes,
        }
    }

    pub fn years(&self) -> &i32 {
        &self.years
    }
    pub fn months(&self) -> &u8 {
        &self.months
    }
    pub fn days(&self) -> &u8 {
        &self.days
    }
    pub fn hours(&self) -> &u8 {
        &self.hours
    }
    pub fn minutes(&self) -> &u8 {
        &self.minutes
    }
}

impl From<i64> for DateTime {
    fn from(value: i64) -> Self {
        Self { unix_secs: value }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn tet() {}
}
