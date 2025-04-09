use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Passenger {

    #[serde(rename = "MissionID")]
    pub mission_id: u64,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "VIP")]
    pub vip: bool,

    #[serde(rename = "Wanted")]
    pub wanted: bool,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Passengers {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Manifest")]
    pub manifest: Vec<Passenger>,
}