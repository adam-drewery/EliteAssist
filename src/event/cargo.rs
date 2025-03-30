use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Cargo {

    pub timestamp: String,

    #[serde(rename = "Vessel")]
    pub vessel: String,

    #[serde(rename = "Count")]
    pub count: i64,

    #[serde(rename = "Inventory")]
    pub inventory: Vec<i32>,
}