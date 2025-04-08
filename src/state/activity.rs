use chrono::{DateTime, Utc};

pub struct GameActivity {
    pub time: DateTime<Utc>,
    pub time_display: String,
    pub verb: String,
    pub noun: String,
}