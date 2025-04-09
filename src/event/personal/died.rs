use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Died {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "KillerName")]
    pub killer_name: Option<String>,

    #[serde(rename = "KillerShip")]
    pub killer_ship: Option<String>,

    #[serde(rename = "KillerRank")]
    pub killer_rank: Option<String>,
}