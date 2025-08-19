use std::collections::HashMap;
use thousands::Separable;
use crate::journal::event;
use crate::{lookup, state};
use crate::journal::format::{prettify_date, title_case};
use crate::lookup::fdev_ids::all_materials;

impl event::Materials {

    fn apply_counts_to(&self, target: &mut Vec<state::MaterialGroup>) {

        let count_map: HashMap<String, u64> = self
            .raw
            .iter()
            .map(|m| (m.name.clone(), m.count))
            .chain(self.manufactured.iter().map(|m| (m.name.clone(), m.count)))
            .chain(self.encoded.iter().map(|m| (m.name.clone(), m.count)))
            .collect();

        for group in target {
            for material in &mut group.materials {
                if let Some(&count) = count_map.get(material.id.as_str()) {
                    material.count = count;
                } else {
                    material.count = 0;
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.encoded.is_empty() && self.raw.is_empty() && self.manufactured.is_empty()
    }
}

impl Into<state::Materials> for event::Materials {
    fn into(self) -> state::Materials {

        let mut materials = all_materials().clone();
        self.apply_counts_to(&mut materials.raw);
        self.apply_counts_to(&mut materials.manufactured);
        self.apply_counts_to(&mut materials.encoded);
        materials
    }
}

impl event::Inventory {
    pub fn is_empty(&self) -> bool {
        self.items.is_none()
            && self.components.is_none()
            && self.consumables.is_none()
            && self.data.is_none()
    }
}

impl Into<state::ShipLocker> for event::Inventory {

    fn into(self) -> state::ShipLocker {

        state::ShipLocker {

            items: map_vec(self.items),
            consumables: self.consumables.unwrap_or_default().into_iter().map(|c| c.into()).collect(),
            data: map_vec(self.data),

            // this one's locations come from the "materials" map instead of "items" so treat it separately
            components: self.components.unwrap_or_default().into_iter().map(|c| {
                state::ShipLockerItem {
                    name: c.name_localised.clone().unwrap_or(title_case(&c.name)),
                    for_mission: c.mission_id.is_some(),
                    count: c.count,
                    locations: lookup::locations_for_material(&c.name_localised.unwrap_or(c.name))
                }
            }).collect(),
        }
    }
}

impl Into<state::ShipLockerItem> for event::Item {

    fn into(self) -> state::ShipLockerItem {

        state::ShipLockerItem {
            name: self.name_localised.clone().unwrap_or(title_case(&self.name)),
            for_mission: self.mission_id.is_some(),
            count: self.count,
            locations: lookup::locations_for_item(&self.name_localised.unwrap_or(self.name))
        }
    }
}

fn group_and_sort(items: Vec<event::Item>) -> Vec<event::Item> {

    let mut grouped_items: HashMap<(String, Option<u64>), event::Item> = HashMap::new();

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

fn map_vec(vec: Option<Vec<event::Item>>) -> Vec<state::ShipLockerItem> {

    group_and_sort(vec.unwrap_or_default())
        .into_iter()
        .map(|f| f.into())
        .collect()
}

impl event::ShipEquipmentPurchase {
    pub fn into(self, item: &str) -> state::GameEventLog {
        state::GameEventLog {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: format!("Bought {} for", item).into(),
            noun: format!("{}CR", &self.cost.to_string().separate_with_commas())
        }
    }
}

impl Into<state::GameEventLog> for event::RestockVehicle {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Restocked vehicles for".into(),
            noun: format!("{}CR", self.cost.to_string().separate_with_commas()),
        }
    }
}

impl Into<state::ShipLockerItem> for event::Consumable {
    fn into(self) -> state::ShipLockerItem {
        state::ShipLockerItem {
            name: self.name_localised.clone().unwrap_or(title_case(&self.name)),
            count: self.count,
            for_mission: false,
            locations: lookup::locations_for_item(&self.name_localised.unwrap_or(self.name))
        }
    }
}