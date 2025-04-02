use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MarketSell {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: i64,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: String,

    #[serde(rename = "Count")]
    pub count: i64,

    #[serde(rename = "SellPrice")]
    pub sell_price: i64,

    #[serde(rename = "TotalSale")]
    pub total_sale: i64,

    #[serde(rename = "AvgPricePaid")]
    pub avg_price_paid: i64,
}