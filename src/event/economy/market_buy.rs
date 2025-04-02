use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MarketBuy {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: i64,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Count")]
    pub count: i64,

    #[serde(rename = "BuyPrice")]
    pub buy_price: i64,

    #[serde(rename = "TotalCost")]
    pub total_cost: i64,
}