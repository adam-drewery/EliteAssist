use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EngineerContribution {

    pub timestamp: String,

    #[serde(rename = "Engineer")]
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: i64,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Quantity")]
    pub quantity: i64,

    #[serde(rename = "TotalQuantity")]
    pub total_quantity: i64,
}