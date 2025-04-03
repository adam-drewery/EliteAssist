use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Powerplay {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "Rank")]
    pub rank: u8,

    #[serde(rename = "Merits")]
    pub merits: u32,

    #[serde(rename = "TimePledged")]
    pub time_pledged: u64
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct PowerplayJoin {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Power")]
    pub power: String,
}