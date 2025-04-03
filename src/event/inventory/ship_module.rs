use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct StoredModule {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "StorageSlot")]
    pub storage_slot: u32,

    #[serde(rename = "StarSystem")]
    pub star_system: Option<String>,

    #[serde(rename = "MarketID")]
    pub market_id: Option<u32>,

    #[serde(rename = "TransferCost")]
    pub transfer_cost: Option<u32>,

    #[serde(rename = "TransferTime")]
    pub transfer_time: Option<u32>,

    #[serde(rename = "BuyPrice")]
    pub buy_price: u32,

    #[serde(rename = "Hot")]
    pub hot: bool,

    #[serde(rename = "EngineerModifications")]
    pub engineer_modifications: Option<String>,

    #[serde(rename = "Level")]
    pub level: Option<u32>,

    #[serde(rename = "Quality")]
    pub quality: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StoredModules {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "Items")]
    pub items: Vec<StoredModule>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleSwap {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "FromSlot")]
    pub from_slot: String,

    #[serde(rename = "ToSlot")]
    pub to_slot: String,

    #[serde(rename = "FromItem")]
    pub from_item: String,

    #[serde(rename = "FromItem_Localised")]
    pub from_item_localised: Option<String>,

    #[serde(rename = "ToItem")]
    pub to_item: String,

    #[serde(rename = "ToItem_Localised")]
    pub to_item_localised: Option<String>,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FetchRemoteModule {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "StorageSlot")]
    pub storage_slot: u32,

    #[serde(rename = "StoredItem")]
    pub stored_item: String,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localised: String,

    #[serde(rename = "ServerId")]
    pub server_id: u64,

    #[serde(rename = "TransferCost")]
    pub transfer_cost: u32,

    #[serde(rename = "TransferTime")]
    pub transfer_time: u32,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}