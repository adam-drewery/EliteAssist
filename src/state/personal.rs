use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct Rank {
    pub timestamp: DateTime<Utc>,
    pub combat: u64,
    pub trade: u64,
    pub explore: u64,
    pub soldier: u64,
    pub exobiologist: u64,
    pub empire: u64,
    pub federation: u64,
    pub cqc: u64
    
}

#[derive(Default)]
pub struct CrimeStats {
    pub legal_state: String,
    pub active_fine: bool,
    pub wanted: bool,
}

#[derive(Default)]
pub struct Reputation {
    pub timestamp: DateTime<Utc>,
    pub empire: f64,
    pub federation: f64,
    pub independent: f64,
    pub alliance: f64
}