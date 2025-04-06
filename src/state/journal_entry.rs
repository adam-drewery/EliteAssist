use chrono::{DateTime, Utc};

pub struct JournalEntry {
    pub time: DateTime<Utc>,
    pub time_display: String,
    pub text: String,
    pub star_system: String,
    pub station: Option<String>,
    pub body: String,
}