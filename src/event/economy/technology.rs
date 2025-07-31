use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct UnlockedItem {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Commodity {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Material {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Category")]
    pub category: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TechnologyBroker {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "BrokerType")]
    pub broker_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "ItemsUnlocked")]
    pub items_unlocked: Vec<UnlockedItem>,

    #[serde(rename = "Commodities")]
    pub commodities: Vec<Commodity>,

    #[serde(rename = "Materials")]
    pub materials: Vec<Material>,
}