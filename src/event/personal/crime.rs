use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CrimeVictim {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Offender")]
    pub offender: String,

    #[serde(rename = "CrimeType")]
    pub crime_type: String,

    #[serde(rename = "Fine")]
    pub fine: Option<u32>,
}