use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct RefuelAll {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Cost")]
    pub cost: u32,

    #[serde(rename = "Amount")]
    pub amount: f64,
}