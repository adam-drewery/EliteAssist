use serde::Deserialize;
use crate::events::ship_locker_component::ShipLockerComponent;
use crate::events::ship_locker_consumable::ShipLockerConsumable;
use crate::events::ship_locker_data::ShipLockerData;
use crate::events::ship_locker_item::ShipLockerItem;

#[derive(Deserialize)]
pub struct ShipLocker {

    pub timestamp: String,

    #[serde(rename = "Items")]
    pub items: Vec<ShipLockerItem>,

    #[serde(rename = "Components")]
    pub components: Vec<ShipLockerComponent>,

    #[serde(rename = "Consumables")]
    pub consumables: Vec<ShipLockerConsumable>,

    #[serde(rename = "Data")]
    pub data: Vec<ShipLockerData>
}