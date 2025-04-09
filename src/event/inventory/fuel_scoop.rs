use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FuelScoop {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Scooped")]
    pub scooped: f64,

    #[serde(rename = "Total")]
    pub total: f64,
}