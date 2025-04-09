use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CommitCrime {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CrimeType")]
    pub crime_type: String,

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Fine")]
    pub fine: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PayFines {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Amount")]
    pub amount: u32,

    #[serde(rename = "AllFines")]
    pub all_fines: bool,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ClearImpound {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}