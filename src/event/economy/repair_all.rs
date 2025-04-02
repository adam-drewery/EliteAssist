use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct RepairAll {

    pub timestamp: String,

    #[serde(rename = "Cost")]
    pub cost: u32,
}