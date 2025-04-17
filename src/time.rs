use chrono::Local;

#[derive(Debug)]
pub struct Time {
    unix_secs: i64,
}

impl Time {
    pub fn now() -> Self {
        Self {
            unix_secs: Local::now().naive_local().and_utc().timestamp(),
        }
    }
}

impl From<i64> for Time {
    fn from(value: i64) -> Self {
        Self { unix_secs: value }
    }
}
