use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CollectItems {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Stolen")]
    pub stolen: bool,
}