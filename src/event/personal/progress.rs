use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Progress {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Combat")]
    pub combat: u8,

    #[serde(rename = "Trade")]
    pub trade: u8,

    #[serde(rename = "Explore")]
    pub explore: u8,

    #[serde(rename = "Soldier")]
    pub soldier: u8,

    #[serde(rename = "Exobiologist")]
    pub exobiologist: u8,

    #[serde(rename = "Empire")]
    pub empire: u8,

    #[serde(rename = "Federation")]
    pub federation: u8,

    #[serde(rename = "CQC")]
    pub cqc: u8
}