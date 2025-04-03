use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FactionKillBond {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Reward")]
    pub reward: u32,

    #[serde(rename = "AwardingFaction")]
    pub awarding_faction: String,

    #[serde(rename = "VictimFaction")]
    pub victim_faction: String,
}