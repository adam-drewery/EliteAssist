use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Missions {

    pub timestamp: String
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MissionAccepted {

    pub timestamp: String,

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
    pub count: Option<i64>,

    #[serde(rename = "DestinationSystem")]
    pub destination_system: Option<String>,

    #[serde(rename = "DestinationSettlement")]
    pub destination_settlement: Option<String>,

    #[serde(rename = "Expiry")]
    pub expiry: String,

    #[serde(rename = "Wing")]
    pub wing: bool,

    #[serde(rename = "Influence")]
    pub influence: String,

    #[serde(rename = "Reputation")]
    pub reputation: String,

    #[serde(rename = "Reward")]
    pub reward: Option<i64>,

    #[serde(rename = "MissionID")]
    pub mission_id: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MissionFailed {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "LocalisedName")]
    pub localised_name: String,

    #[serde(rename = "MissionID")]
    pub mission_id: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MissionAbandoned {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "LocalisedName")]
    pub localised_name: String,

    #[serde(rename = "MissionID")]
    pub mission_id: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SystemInfluence {

    #[serde(rename = "SystemAddress")]
    pub system_address: i64,

    #[serde(rename = "Trend")]
    pub trend: String,

    #[serde(rename = "Influence")]
    pub influence: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Effect {

    #[serde(rename = "Effect")]
    pub effect: String,

    #[serde(rename = "Effect_Localised")]
    pub effect_localised: String,

    #[serde(rename = "Trend")]
    pub trend: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
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

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MissionCompleted {

    pub timestamp: String,

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "LocalisedName")]
    pub localised_name: String,

    #[serde(rename = "MissionID")]
    pub mission_id: i64,

    #[serde(rename = "DestinationSystem")]
    pub destination_system: Option<String>,

    #[serde(rename = "DestinationStation")]
    pub destination_station: Option<String>,

    #[serde(rename = "Reward")]
    pub reward: Option<i64>,

    #[serde(rename = "FactionEffects")]
    pub faction_effects: Vec<FactionEffect>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MissionRedirected {

    pub timestamp: String,

    #[serde(rename = "MissionID")]
    pub mission_id: i64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "LocalisedName")]
    pub localised_name: String,

    #[serde(rename = "NewDestinationStation")]
    pub new_destination_station: String,

    #[serde(rename = "NewDestinationSystem")]
    pub new_destination_system: String,

    #[serde(rename = "OldDestinationStation")]
    pub old_destination_station: String,

    #[serde(rename = "OldDestinationSystem")]
    pub old_destination_system: String,
}