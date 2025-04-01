use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct RestockVehicle {

    pub timestamp: String,

    #[serde(rename = "Type")]
    pub type_: String,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "Cost")]
    pub cost: u64,

    #[serde(rename = "Count")]
    pub count: u64
}