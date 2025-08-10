use chrono::{DateTime, Utc};

pub struct GameEventLog {
    pub time: DateTime<Utc>,
    pub time_display: String,
    pub verb: String,
    pub noun: String,
}