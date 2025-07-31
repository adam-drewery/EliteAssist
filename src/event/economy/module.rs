use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
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
    pub buy_item_localised: Option<String>,

    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,

    #[serde(rename = "BuyPrice")]
    pub buy_price: u32,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Clone, Debug, Deserialize)]
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
    pub retrieved_item_localised: Option<String>,

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

#[derive(Clone, Debug, Deserialize)]
pub struct ModuleSell {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "SellItem")]
    pub sell_item: String,

    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localised: Option<String>,

    #[serde(rename = "SellPrice")]
    pub sell_price: u32,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModuleSellRemote {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "StorageSlot")]
    pub storage_slot: u32,

    #[serde(rename = "SellItem")]
    pub sell_item: String,

    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localised: Option<String>,

    #[serde(rename = "ServerId")]
    pub server_id: u64,

    #[serde(rename = "SellPrice")]
    pub sell_price: u32,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
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
    pub stored_item_localised: Option<String>,

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

#[derive(Clone, Debug, Deserialize)]
pub struct ModuleItem {
    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Hot")]
    pub hot: bool,

    #[serde(rename = "EngineerModifications")]
    pub engineer_modifications: Option<String>,

    #[serde(rename = "Level")]
    pub level: Option<u32>,

    #[serde(rename = "Quality")]
    pub quality: Option<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MassModuleStore {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipId")]
    pub ship_id: u64,

    #[serde(rename = "Items")]
    pub items: Vec<ModuleItem>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StationTransferRequest {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "StorageSlot")]
    pub storage_slot: String,

    #[serde(rename = "StoredItem")]
    pub stored_item: String,

    #[serde(rename = "ServerId")]
    pub server_id: String,

    #[serde(rename = "TransferCost")]
    pub transfer_cost: u32,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipId")]
    pub ship_id: u32,

    #[serde(rename = "TransferTime")]
    pub transfer_time: u32,
}