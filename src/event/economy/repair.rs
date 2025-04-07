use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Repair {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Items")]
    pub items: Vec<String>,

    #[serde(rename = "Cost")]
    pub cost: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RepairAll {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Cost")]
    pub cost: u32,
}