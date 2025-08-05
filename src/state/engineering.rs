use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct EngineerProgress {
    pub timestamp: DateTime<Utc>,
    pub engineers: Vec<Engineer>,
}

pub struct Engineer {
    pub engineer: String,
    pub engineer_id: u64,
    pub progress: String,
    pub rank_progress: u64,
    pub rank: u64,
}
