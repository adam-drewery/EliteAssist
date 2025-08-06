use crate::state;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

macro_rules! static_hashmap {
    ($file_path:expr, $type:ty) => {{
        static MAP: Lazy<HashMap<String, $type>> = Lazy::new(|| {
            let csv_bytes: &[u8] = include_bytes!($file_path);
            let mut rdr = csv::Reader::from_reader(csv_bytes);
            let mut map = HashMap::new();
            for result in rdr.deserialize() {
                let record: $type = result.unwrap();
                map.insert(record.symbol.to_lowercase(), record);
            }
            map
        });
        &MAP
    }};
}

#[derive(Debug, Deserialize, Default)]
pub struct Outfitting {
    pub id: String,
    pub symbol: String,
    pub category: String,
    pub name: String,
    pub mount: String,
    pub guidance: String,
    pub ship: String,
    pub class: String,
    pub rating: String,
    pub entitlement: String,
}

impl Outfitting {
    pub(crate) fn new() -> Outfitting {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
pub struct Shipyard {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub entitlement: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Material {
    pub id: String,
    pub symbol: String,
    pub rarity: String,
    #[serde(rename = "type")] pub r#type: String,
    pub category: String,
    pub name: String,
}

impl Outfitting {
    pub fn metadata(name: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/outfitting.csv", Outfitting).get(&name.to_lowercase())
    }
}

impl Shipyard {
    pub fn metadata(name: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/shipyard.csv", Shipyard).get(&name.to_lowercase())
    }
}

impl Material {
    pub fn metadata(name: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/material.csv", Material).get(&name.to_lowercase())
    }
}

pub fn all_materials() -> state::Materials {
    let materials = static_hashmap!("../fdev-ids/material.csv", Material);
    let mut raw = HashMap::new();
    let mut encoded = HashMap::new();
    let mut manufactured = HashMap::new();

    for material in materials.values() {
        let target = match material.r#type.as_str() {
            "Raw" => &mut raw,
            "Encoded" => &mut encoded,
            "Manufactured" => &mut manufactured,
            _ => continue
        };

        target.entry(material.category.clone())
            .or_insert_with(Vec::new)
            .push(material.into());
    }

    let to_sorted_groups = |map: HashMap<String, Vec<state::Material>>, name_fn: fn(&str) -> String| {
        let mut groups: Vec<_> = map.into_iter()
            .map(|(name, mut materials)| {
                materials.sort_by_key(|m| m.name.clone());
                state::MaterialGroup {
                    name: name_fn(&name),
                    materials,
                }
            })
            .collect();
        groups.sort_by_key(|g| g.name.clone());
        groups
    };

    state::Materials {
        encoded: to_sorted_groups(encoded, |s| s.to_string()),
        manufactured: to_sorted_groups(manufactured, |s| s.to_string()),
        raw: to_sorted_groups(raw, apply_name),
    }
}

impl Into<state::Material> for &Material {
    fn into(self) -> state::Material {
        state::Material {
            id: self.symbol.to_lowercase(),
            name: self.name.clone(),
            rarity: self.rarity.parse().unwrap(),
            count: 0,
            locations: Vec::new(),
        }
    }
}

fn apply_name(input: &str) -> String {
    CATEGORY_NAMES
        .get(input)
        .copied()
        .unwrap_or(input)
        .to_string()
}

static CATEGORY_NAMES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("1", "Light Metals and Metalloids");
    m.insert("2", "Reactive Nonmetals and Transition Metals");
    m.insert("3", "Chalcogens and Transition Metals");
    m.insert("4", "Base Metals and Post-Transition Metals");
    m.insert("5", "Coinage and Industrial Metals");
    m.insert("6", "Heavy Metals and Metalloids");
    m.insert("7", "Diverse Utility Elements");
    m
});
