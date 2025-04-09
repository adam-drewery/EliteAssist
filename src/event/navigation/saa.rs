use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SAAScanComplete {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "BodyName")]
    pub body_name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "ProbesUsed")]
    pub probes_used: u32,

    #[serde(rename = "EfficiencyTarget")]
    pub efficiency_target: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Genus {

    #[serde(rename = "Genus")]
    pub genus: String,

    #[serde(rename = "Genus_Localised")]
    pub genus_localised: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Signal {

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SAASignalsFound {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "BodyName")]
    pub body_name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "Signals")]
    pub signals: Vec<Signal>,

    #[serde(rename = "Genuses")]
    pub genuses: Vec<Genus>,
}