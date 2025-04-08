use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct Rank {
    pub timestamp: DateTime<Utc>,
    pub combat: u8,
    pub trade: u8,
    pub explore: u8,
    pub soldier: u8,
    pub exobiologist: u8,
    pub empire: u8,
    pub federation: u8,
    pub cqc: u8
    
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