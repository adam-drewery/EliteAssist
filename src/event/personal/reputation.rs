use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Reputation {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Empire")]
    pub empire: f64,

    #[serde(rename = "Federation")]
    pub federation: f64,

    #[serde(rename = "Independent")]
    pub independent: f64,

    #[serde(rename = "Alliance")]
    pub alliance: f64
}