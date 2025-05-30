use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ReservoirReplenished {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "FuelMain")]
    pub fuel_main: f64,

    #[serde(rename = "FuelReservoir")]
    pub fuel_reservoir: f64,
}