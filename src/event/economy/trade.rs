use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct TradedMaterial {

    #[serde(rename = "Material")]
    pub material: String,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Quantity")]
    pub quantity: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MaterialTrade {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "TraderType")]
    pub trader_type: String,

    #[serde(rename = "Paid")]
    pub paid: TradedMaterial,

    #[serde(rename = "Received")]
    pub received: TradedMaterial,
}