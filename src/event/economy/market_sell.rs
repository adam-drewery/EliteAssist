use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MarketSell {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "SellPrice")]
    pub sell_price: u32,

    #[serde(rename = "TotalSale")]
    pub total_sale: u32,

    #[serde(rename = "AvgPricePaid")]
    pub avg_price_paid: u64,
}