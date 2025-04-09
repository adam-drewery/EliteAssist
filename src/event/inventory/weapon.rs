use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct UpgradeResource {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpgradeWeapon {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Class")]
    pub class: u32,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,

    #[serde(rename = "Cost")]
    pub cost: u32,

    #[serde(rename = "Resources")]
    pub resources: Vec<UpgradeResource>,
}