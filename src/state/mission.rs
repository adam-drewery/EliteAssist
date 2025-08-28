use crate::journal::event;
use chrono::{DateTime, Utc};

pub struct Mission {

    pub faction: Box<str>,
    pub name: Box<str>,
    pub commodity: Option<Box<str>>,
    pub count: Option<u64>,
    pub destination_system: Option<Box<str>>,
    pub destination_settlement: Option<Box<str>>,
    pub expiry: Option<DateTime<Utc>>,
    pub wing: bool,
    pub influence: Box<str>,
    pub reputation: Box<str>,
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
