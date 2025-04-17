use chrono::Local;

pub const DEFAULT_FORMAT: &str = "%Y/%m/%d %H:%M";

#[derive(Debug)]
pub struct FakeUtcTime {
    unix_secs: i64,
}

impl FakeUtcTime {
    pub fn now() -> Self {
        Self {
            unix_secs: Local::now().naive_local().and_utc().timestamp(),
        }
    }
}

impl From<i64> for FakeUtcTime {
    fn from(value: i64) -> Self {
        Self { unix_secs: value }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn tet() {}
}
