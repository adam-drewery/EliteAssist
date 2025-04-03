use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FighterDestroyed {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ID")]
    pub id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FighterRebuilt {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DockFighter {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ID")]
    pub id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewLaunchFighter {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,

    #[serde(rename = "Crew")]
    pub crew: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct LaunchFighter {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u64,

    #[serde(rename = "PlayerControlled")]
    pub player_controlled: bool,
}