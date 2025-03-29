use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Mining {

    #[serde(rename = "Mining_Profits")]
    pub mining_profits: u64,

    #[serde(rename = "Quantity_Mined")]
    pub quantity_mined: u64,

    #[serde(rename = "Materials_Collected")]
    pub materials_collected: u64,
}