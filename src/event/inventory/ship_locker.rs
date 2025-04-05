use crate::text::title_case;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

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