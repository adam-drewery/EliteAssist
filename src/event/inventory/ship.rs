use crate::text::title_case;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct ShipLocker {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Items")]
    pub items: Option<Vec<ShipLockerItem>>,

    #[serde(rename = "Components")]
    pub components: Option<Vec<ShipLockerItem>>,

    #[serde(rename = "Consumables")]
    pub consumables: Option<Vec<ShipLockerItem>>,

    #[serde(rename = "Data")]
    pub data: Option<Vec<ShipLockerItem>>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ShipLockerItem {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    #[serde(rename = "MissionID")]
    pub mission_id: Option<u64>,

    #[serde(rename = "Count")]
    pub count: u64,
}

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

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
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

impl ShipLocker {
    pub fn is_empty(&self) -> bool {
        self.items.is_none()
            && self.components.is_none()
            && self.consumables.is_none()
            && self.data.is_none()
    }
}

impl Into<crate::state::ShipLocker> for ShipLocker {

    fn into(self) -> crate::state::ShipLocker {

        crate::state::ShipLocker {

            items: map_vec(self.items),
            consumables: map_vec(self.consumables),
            data: map_vec(self.data),
            components: map_vec(self.components),
        }
    }
}

impl Into<crate::state::ShipLockerItem> for ShipLockerItem {

    fn into(self) -> crate::state::ShipLockerItem {

        crate::state::ShipLockerItem {
            name: self.name_localised.unwrap_or(title_case(&self.name)),
            for_mission: self.mission_id.is_some(),
            count: self.count,
        }
    }
}

fn group_and_sort(items: Vec<ShipLockerItem>) -> Vec<ShipLockerItem> {

    let mut grouped_items: HashMap<(String, Option<u64>), ShipLockerItem> = HashMap::new();

    for item in items {
        grouped_items
            .entry((item.name.clone(), item.mission_id))
            .and_modify(|e| e.count += item.count)
            .or_insert(item);
    }

    let mut items: Vec<_> = grouped_items.into_values().collect();
    items.sort_by(|a, b| a.name.cmp(&b.name));
    items
}

fn map_vec(vec: Option<Vec<ShipLockerItem>>) -> Vec<crate::state::ShipLockerItem> {

    group_and_sort(vec.unwrap_or_default())
        .into_iter()
        .map(|f| f.into())
        .collect()
}