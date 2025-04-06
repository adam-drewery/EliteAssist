use chrono::{DateTime, Utc};

pub struct JournalEntry {
    pub time: DateTime<Utc>,
    pub time_display: String,
    pub verb: String,
    pub noun: String,
}