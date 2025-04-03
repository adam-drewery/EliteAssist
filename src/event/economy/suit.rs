use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Suit {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Price")]
    pub price: i64,

    #[serde(rename = "SuitID")]
    pub suit_id: i64,

    #[serde(rename = "SuitMods")]
    pub suit_mods: Vec<String>,
}