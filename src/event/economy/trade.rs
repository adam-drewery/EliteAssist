use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct TradedMaterial {

    #[serde(rename = "Material")]
    pub material: String,

    #[serde(rename = "Quantity")]
    pub quantity: u32,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct ScientificResearch {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Count")]
    pub count: u32,
}