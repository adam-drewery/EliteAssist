use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Repair {

    pub timestamp: String,

    #[serde(rename = "Items")]
    pub items: Vec<String>,

    #[serde(rename = "Cost")]
    pub cost: i64,
}