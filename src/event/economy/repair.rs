use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Repair {

    pub timestamp: String,

    #[serde(rename = "Items")]
    pub items: Vec<String>,

    #[serde(rename = "Cost")]
    pub cost: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct RepairAll {

    pub timestamp: String,

    #[serde(rename = "Cost")]
    pub cost: u32,
}