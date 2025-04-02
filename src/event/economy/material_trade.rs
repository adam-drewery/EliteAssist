use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct TradedMaterial {

    #[serde(rename = "Material")]
    pub material: String,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Quantity")]
    pub quantity: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MaterialTrade {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: i64,

    #[serde(rename = "TraderType")]
    pub trader_type: String,

    #[serde(rename = "Paid")]
    pub paid: TradedMaterial,

    #[serde(rename = "Received")]
    pub received: TradedMaterial,
}