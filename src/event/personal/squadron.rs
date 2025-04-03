use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct AppliedToSquardon {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SquadronStartup {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,

    #[serde(rename = "CurrentRank")]
    pub current_rank: u8,
}