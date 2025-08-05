use crate::{event, state};

impl Into<state::Mission> for event::MissionAccepted {
    fn into(self) -> state::Mission {
        state::Mission {
            name: self.localised_name,
            mission_id: self.mission_id,
            faction: self.faction,
            commodity: self.commodity_localised,
            count: self.count,
            destination_system: self.destination_system,
            destination_settlement: self.destination_settlement,
            expiry: self.expiry,
            wing: self.wing,
            influence: self.influence,
            reputation: self.reputation,
            reward: self.reward,
        }
    }
}