use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BookTaxi {#[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Cost")]
    pub cost: u32,

    #[serde(rename = "DestinationSystem")]
    pub destination_system: String,

    #[serde(rename = "DestinationLocation")]
    pub destination_location: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CancelTaxi {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Refund")]
    pub refund: u32,
}