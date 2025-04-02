use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MarketBuy {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "BuyPrice")]
    pub buy_price: u32,

    #[serde(rename = "TotalCost")]
    pub total_cost: u32,
}