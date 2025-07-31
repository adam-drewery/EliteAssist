use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::state;

#[derive(Clone, Debug, Deserialize)]
pub struct MissionAccepted {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "LocalisedName")]
    pub localised_name: String,

    #[serde(rename = "Commodity")]
    pub commodity: Option<String>,

    #[serde(rename = "Commodity_Localised")]
    pub commodity_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: Option<u32>,

    #[serde(rename = "DestinationSystem")]
    pub destination_system: Option<String>,

    #[serde(rename = "DestinationSettlement")]
    pub destination_settlement: Option<String>,

    #[serde(rename = "Expiry")]
    pub expiry: Option<String>,

    #[serde(rename = "Wing")]
    pub wing: bool,

    #[serde(rename = "Influence")]
    pub influence: String,

    #[serde(rename = "Reputation")]
    pub reputation: String,

    #[serde(rename = "Reward")]
    pub reward: Option<u32>,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mission {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "LocalisedName")]
    pub localised_name: Option<String>,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SystemInfluence {

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Trend")]
    pub trend: String,

    #[serde(rename = "Influence")]
    pub influence: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Effect {

    #[serde(rename = "Effect")]
    pub effect: String,

    #[serde(rename = "Effect_Localised")]
    pub effect_localised: Option<String>,

    #[serde(rename = "Trend")]
    pub trend: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FactionEffect {

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Effects")]
    pub effects: Vec<Effect>,

    #[serde(rename = "Influence")]
    pub influence: Vec<SystemInfluence>,

    #[serde(rename = "ReputationTrend")]
    pub reputation_trend: String,

    #[serde(rename = "Reputation")]
    pub reputation: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MissionCompleted {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "LocalisedName")]
    pub localised_name: Option<String>,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,

    #[serde(rename = "DestinationSystem")]
    pub destination_system: Option<String>,

    #[serde(rename = "DestinationStation")]
    pub destination_station: Option<String>,

    #[serde(rename = "Reward")]
    pub reward: Option<u32>,

    #[serde(rename = "FactionEffects")]
    pub faction_effects: Vec<FactionEffect>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MissionRedirected {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "LocalisedName")]
    pub localised_name: Option<String>,

    #[serde(rename = "LocalisedName_Localised")]
    pub localised_name_localised: Option<String>,

    #[serde(rename = "NewDestinationStation")]
    pub new_destination_station: String,

    #[serde(rename = "NewDestinationSystem")]
    pub new_destination_system: String,

    #[serde(rename = "OldDestinationStation")]
    pub old_destination_station: String,

    #[serde(rename = "OldDestinationSystem")]
    pub old_destination_system: String,
}

impl Into<state::Mission> for MissionAccepted {
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

#[derive(Clone, Debug, Deserialize)]
pub struct CargoDepot {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MissionID")]
    pub mission_id: u64,

    #[serde(rename = "UpdateType")]
    pub update_type: String,

    #[serde(rename = "CargoType")]
    pub cargo_type: String,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "StartMarketID")]
    pub start_market_id: u64,

    #[serde(rename = "EndMarketID")]
    pub end_market_id: u64,

    #[serde(rename = "ItemsCollected")]
    pub items_collected: u32,

    #[serde(rename = "ItemsDelivered")]
    pub items_delivered: u32,

    #[serde(rename = "TotalItemsToDeliver")]
    pub total_items_to_deliver: u32,

    #[serde(rename = "Progress")]
    pub progress: u32,
}