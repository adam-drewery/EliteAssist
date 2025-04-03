use crate::text::title_case;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
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

impl ShipLockerItem {
    pub fn display_name(&self) -> String {
        self.name_localised.clone().unwrap_or(title_case(&self.name))
    }
}