use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BuyDrones {

    pub timestamp: String,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "BuyPrice")]
    pub buy_price: u32,

    #[serde(rename = "TotalCost")]
    pub total_cost: u32,
}