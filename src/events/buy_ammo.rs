use serde::Deserialize;

#[derive(Deserialize)]
pub struct BuyAmmo {

    pub timestamp: String,

    #[serde(rename = "Cost")]
    pub cost: u64
}