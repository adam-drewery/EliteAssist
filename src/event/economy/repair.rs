use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Repair {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Item")]
    pub item: Option<String>,

    #[serde(rename = "Items")]
    pub items: Option<Vec<String>>,

    #[serde(rename = "Cost")]
    pub cost: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RepairAll {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Cost")]
    pub cost: u32,
}