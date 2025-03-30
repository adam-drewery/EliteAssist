use crate::event::ship_locker_item::ShipLockerItem;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ShipLocker {

    pub timestamp: String,

    #[serde(rename = "Items")]
    pub items: Option<Vec<ShipLockerItem>>,

    #[serde(rename = "Components")]
    pub components: Option<Vec<ShipLockerItem>>,

    #[serde(rename = "Consumables")]
    pub consumables: Option<Vec<ShipLockerItem>>,

    #[serde(rename = "Data")]
    pub data: Option<Vec<ShipLockerItem>>
}