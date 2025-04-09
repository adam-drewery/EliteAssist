use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FileHeader {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    pub part: u8,

    pub language: String,

    #[serde(rename = "gameversion")]
    pub game_version: String,

    pub build: String
}