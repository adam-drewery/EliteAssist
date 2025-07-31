use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BuyDrones {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "BuyPrice")]
    pub buy_price: u32,

    #[serde(rename = "TotalCost")]
    pub total_cost: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SellDrones {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "SellPrice")]
    pub sell_price: u32,

    #[serde(rename = "TotalSale")]
    pub total_sale: u32,
}