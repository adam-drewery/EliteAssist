use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct RefuelAll {

    pub timestamp: String,

    #[serde(rename = "Cost")]
    pub cost: u32,

    #[serde(rename = "Amount")]
    pub amount: f64,
}