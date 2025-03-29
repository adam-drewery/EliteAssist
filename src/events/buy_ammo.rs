use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct BuyAmmo {

    pub timestamp: String,

    #[serde(rename = "Cost")]
    pub cost: u64
}