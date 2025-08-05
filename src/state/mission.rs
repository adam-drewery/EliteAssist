use chrono::{DateTime, Utc};

pub struct Mission {

    pub faction: String,
    pub name: String,
    pub commodity: Option<String>,
    pub count: Option<u64>,
    pub destination_system: Option<String>,
    pub destination_settlement: Option<String>,
    pub expiry: Option<DateTime<Utc>>,
    pub wing: bool,
    pub influence: String,
    pub reputation: String,
    pub reward: Option<u64>,
    pub mission_id: u64,
}