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
    pub reward: i64,

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