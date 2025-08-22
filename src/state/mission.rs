use crate::journal::event;
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

impl From<event::MissionAccepted> for Mission {
    fn from(value: event::MissionAccepted) -> Self {
        Mission {
            name: value.localised_name,
            mission_id: value.mission_id,
            faction: value.faction,
            commodity: value.commodity_localised,
            count: value.count,
            destination_system: value.destination_system,
            destination_settlement: value.destination_settlement,
            expiry: value.expiry,
            wing: value.wing,
            influence: value.influence,
            reputation: value.reputation,
            reward: value.reward,
        }
    }
}
