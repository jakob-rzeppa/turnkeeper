#[derive(Debug, Clone, PartialEq)]
pub struct DateTime {
    value: chrono::DateTime<chrono::Local>,
}

impl DateTime {
    pub fn now() -> Self {
        Self {
            value: chrono::Local::now(),
        }
    }
}
