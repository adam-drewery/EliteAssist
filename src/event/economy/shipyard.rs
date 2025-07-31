use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ShipyardBuy {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,

    #[serde(rename = "ShipPrice")]
    pub ship_price: u32,

    #[serde(rename = "StoreOldShip")]
    pub store_old_ship: String,

    #[serde(rename = "StoreShipID")]
    pub store_ship_id: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ShipyardNew {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,

    #[serde(rename = "NewShipID")]
    pub new_ship_id: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ShipyardSell {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "SellShipID")]
    pub sell_ship_id: u64,

    #[serde(rename = "ShipPrice")]
    pub ship_price: u32,

    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SellShipOnRebuy {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "System")]
    pub system: String,

    #[serde(rename = "SellShipId")]
    pub sell_ship_id: u64,

    #[serde(rename = "ShipPrice")]
    pub ship_price: u32,
}