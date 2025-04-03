use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct RestockVehicle {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Type")]
    pub type_: String,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "Cost")]
    pub cost: u64,

    #[serde(rename = "Count")]
    pub count: u64
}