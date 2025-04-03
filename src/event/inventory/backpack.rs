use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BackpackItem {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Type")]
    pub r#type: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BackpackChange {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Added")]
    pub added: Option<Vec<BackpackItem>>,

    #[serde(rename = "Removed")]
    pub removed: Option<Vec<BackpackItem>>,
}