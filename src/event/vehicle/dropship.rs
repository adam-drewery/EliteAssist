use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CancelDropship {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Refund")]
    pub refund: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DropshipDeploy {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "OnStation")]
    pub on_station: bool,

    #[serde(rename = "OnPlanet")]
    pub on_planet: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BookDropship {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Retreat")]
    pub retreat: bool,

    #[serde(rename = "Cost")]
    pub cost: u32,

    #[serde(rename = "DestinationSystem")]
    pub destination_system: String,

    #[serde(rename = "DestinationLocation")]
    pub destination_location: String,
}
