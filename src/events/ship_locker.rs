use serde::Deserialize;
use crate::events::ship_locker_component::ShipLockerComponent;
use crate::events::ship_locker_consumable::ShipLockerConsumable;
use crate::events::ship_locker_data::ShipLockerData;
use crate::events::ship_locker_item::ShipLockerItem;

#[derive(Deserialize, Debug, Default)]
pub struct ShipLocker {

    pub timestamp: String,

    #[serde(rename = "Items")]
    pub items: Option<Vec<ShipLockerItem>>,

    #[serde(rename = "Components")]
    pub components: Option<Vec<ShipLockerComponent>>,

    #[serde(rename = "Consumables")]
    pub consumables: Option<Vec<ShipLockerConsumable>>,

    #[serde(rename = "Data")]
    pub data: Option<Vec<ShipLockerData>>
}