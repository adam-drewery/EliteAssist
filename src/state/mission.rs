use ed_journals::logs::mission_accepted_event::MissionAcceptedEvent;
use ed_journals::mixed::MixedCommodity;

pub struct Mission {

    pub faction: String,
    pub name: String,
    pub commodity: Option<String>,
    pub count: Option<u16>,
    pub destination_system: Option<String>,
    pub destination_settlement: Option<String>,
    pub expiry: Option<String>,
    pub wing: bool,
    pub influence: String,
    pub reputation: String,
    pub reward: Option<u64>,
    pub mission_id: u64,
}

impl Mission {
    pub fn from(value: MissionAcceptedEvent) -> Self {
        Mission {
            name: value.localized_name.unwrap_or(value.name.0),
            mission_id: value.mission_id,
            faction: value.faction,
            commodity: value.commodity.map(|c| match c {
                MixedCommodity::ShipCommodity(x) => { x.to_string() }
                MixedCommodity::OdysseyItem(x) => { x.to_string() }
            }),
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
