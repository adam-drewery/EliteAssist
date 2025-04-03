use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleBuy {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "StoredItem")]
    pub stored_item: Option<String>,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localised: Option<String>,

    #[serde(rename = "BuyItem")]
    pub buy_item: String,

    #[serde(rename = "BuyItem_Localised")]
    pub buy_item_localised: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "BuyPrice")]
    pub buy_price: u32,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleRetrieve {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "RetrievedItem")]
    pub retrieved_item: String,

    #[serde(rename = "RetrievedItem_Localised")]
    pub retrieved_item_localised: String,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "Hot")]
    pub hot: bool,

    #[serde(rename = "EngineerModifications")]
    pub engineer_modifications: Option<String>,

    #[serde(rename = "Level")]
    pub level: Option<u32>,

    #[serde(rename = "Quality")]
    pub quality: Option<f64>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleSell {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "SellItem")]
    pub sell_item: String,

    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localised: String,

    #[serde(rename = "SellPrice")]
    pub sell_price: u32,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleSellRemote {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "StorageSlot")]
    pub storage_slot: u32,

    #[serde(rename = "SellItem")]
    pub sell_item: String,

    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localised: String,

    #[serde(rename = "ServerId")]
    pub server_id: u64,

    #[serde(rename = "SellPrice")]
    pub sell_price: u32,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleStore {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "StoredItem")]
    pub stored_item: String,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localised: String,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "Hot")]
    pub hot: bool,

    #[serde(rename = "EngineerModifications")]
    pub engineer_modifications: Option<String>,

    #[serde(rename = "Level")]
    pub level: Option<u32>,

    #[serde(rename = "Quality")]
    pub quality: Option<f64>,
}