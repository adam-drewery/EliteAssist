use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Interdiction {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Success")]
    pub success: bool,

    #[serde(rename = "IsPlayer")]
    pub is_player: bool,

    #[serde(rename = "Faction")]
    pub faction: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EscapeInterdiction {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Interdictor")]
    pub interdictor: String,

    #[serde(rename = "IsPlayer")]
    pub is_player: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Interdicted {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Submitted")]
    pub submitted: bool,

    #[serde(rename = "Interdictor")]
    pub interdictor: Option<String>,

    #[serde(rename = "IsPlayer")]
    pub is_player: bool,

    #[serde(rename = "Faction")]
    pub faction: Option<String>,
}