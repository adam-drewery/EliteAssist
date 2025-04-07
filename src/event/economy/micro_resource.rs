use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct BuyMicroResources {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,
    
    #[serde(rename = "Name")]
    pub name: String,
    
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,
    
    #[serde(rename = "Category")]
    pub category: String,
    
    #[serde(rename = "Count")]
    pub count: u32,
    
    #[serde(rename = "Price")]
    pub price: u32,
    
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MicroResource {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Count")]
    pub count: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DeliverPowerMicroResources {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "TotalCount")]
    pub total_count: i64,

    #[serde(rename = "MicroResources")]
    pub micro_resources: Vec<MicroResource>,

    #[serde(rename = "MarketID")]
    pub market_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OfferedMicroResource {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TradeMicroResources {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Offered")]
    pub offered: Vec<OfferedMicroResource>,

    #[serde(rename = "TotalCount")]
    pub total_count: u32,

    #[serde(rename = "Received")]
    pub received: String,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SellMicroResources {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "TotalCount")]
    pub total_count: u32,

    #[serde(rename = "MicroResources")]
    pub micro_resources: Vec<MicroResource>,

    #[serde(rename = "Price")]
    pub price: u32,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}