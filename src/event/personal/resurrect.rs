use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Resurrect {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Option")]
    pub option: String,

    #[serde(rename = "Cost")]
    pub cost: u32,

    #[serde(rename = "Bankrupt")]
    pub bankrupt: bool,
}