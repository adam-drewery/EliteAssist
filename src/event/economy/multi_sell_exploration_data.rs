use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SystemExplorationData {

    #[serde(rename = "SystemName")]
    pub system_name: String,

    #[serde(rename = "NumBodies")]
    pub num_bodies: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MultiSellExplorationData {

    pub timestamp: String,

    #[serde(rename = "Discovered")]
    pub discovered: Vec<SystemExplorationData>,

    #[serde(rename = "BaseValue")]
    pub base_value: u32,

    #[serde(rename = "Bonus")]
    pub bonus: u32,

    #[serde(rename = "TotalEarnings")]
    pub total_earnings: u32,
}