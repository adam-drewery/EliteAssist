use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CargoItem {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Stolen")]
    pub stolen: u32,

    #[serde(rename = "MissionID")]
    pub mission_id: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Cargo {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Vessel")]
    pub vessel: String,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Inventory")]
    pub inventory: Option<Vec<CargoItem>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CollectCargo {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: Option<String>,

    #[serde(rename = "Stolen")]
    pub stolen: bool,

    #[serde(rename = "MissionID")]
    pub mission_id: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EjectCargo {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Abandoned")]
    pub abandoned: bool,
}