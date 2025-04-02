use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SystemExplorationData {

    #[serde(rename = "SystemName")]
    pub system_name: String,

    #[serde(rename = "NumBodies")]
    pub num_bodies: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MultiSellExplorationData {

    pub timestamp: String,

    #[serde(rename = "Discovered")]
    pub discovered: Vec<SystemExplorationData>,

    #[serde(rename = "BaseValue")]
    pub base_value: i64,

    #[serde(rename = "Bonus")]
    pub bonus: i64,

    #[serde(rename = "TotalEarnings")]
    pub total_earnings: i64,
}