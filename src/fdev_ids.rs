use crate::{state, material_locations};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

macro_rules! static_hashmap {
    ($file_path:expr, $type:ty, $field:ident) => {{
        static MAP: Lazy<HashMap<String, $type>> = Lazy::new(|| {
            let csv_bytes: &[u8] = include_bytes!($file_path);
            let mut rdr = csv::Reader::from_reader(csv_bytes);
            let mut map = HashMap::new();
            for result in rdr.deserialize() {
                let record: $type = result.unwrap();
                map.insert(record.$field.to_lowercase(), record);
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

#[derive(Debug, Deserialize)]
pub struct Shipyard {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub entitlement: String,
}

#[derive(Debug, Deserialize)]
pub struct Material {
    pub id: String,
    pub symbol: String,
    pub rarity: String,
    #[serde(rename = "type")] pub r#type: String,
    pub category: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Rank {
    pub number: String,
    pub name: String
}

fn insert_rank(map: &mut HashMap<String, Rank>, number: &str, name: &str) {
    map.insert(
        number.to_string(),
        Rank {
            number: number.to_string(),
            name: name.to_string(),
        },
    );
}

// Hard-coded rank maps for Exobiologist and Mercenary (Odyssey). No idea why they're not included in fdev-ids.
static EXOBIOLOGIST_RANKS: Lazy<HashMap<String, Rank>> = Lazy::new(|| {
    let mut m = HashMap::new();
    insert_rank(&mut m, "0", "Directionless");
    insert_rank(&mut m, "1", "Mostly Directionless");
    insert_rank(&mut m, "2", "Compiler");
    insert_rank(&mut m, "3", "Collector");
    insert_rank(&mut m, "4", "Cataloguer");
    insert_rank(&mut m, "5", "Taxonomist");
    insert_rank(&mut m, "6", "Ecologist");
    insert_rank(&mut m, "7", "Geneticist");
    insert_rank(&mut m, "8", "Elite");
    m
});

static MERCENARY_RANKS: Lazy<HashMap<String, Rank>> = Lazy::new(|| {
    let mut m = HashMap::new();
    insert_rank(&mut m, "0", "Defenceless");
    insert_rank(&mut m, "1", "Mostly Defenceless");
    insert_rank(&mut m, "2", "Rookie");
    insert_rank(&mut m, "3", "Soldier");
    insert_rank(&mut m, "4", "Gunslinger");
    insert_rank(&mut m, "5", "Warrior");
    insert_rank(&mut m, "6", "Gladiator");
    insert_rank(&mut m, "7", "Deadeye");
    insert_rank(&mut m, "8", "Elite");
    m
});

impl Outfitting {
    pub fn metadata(name: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/outfitting.csv", Outfitting, symbol).get(&name.to_lowercase())
    }
}

impl Shipyard {
    pub fn metadata(name: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/shipyard.csv", Shipyard, symbol).get(&name.to_lowercase())
    }
}

impl Material {
    pub fn metadata(name: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/material.csv", Material, symbol).get(&name.to_lowercase())
    }
}

impl Rank {
    pub fn cqc(name: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/CQCRank.csv", Rank, number).get(&name.to_lowercase())
    }

    pub fn combat(name: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/combatrank.csv", Rank, number).get(&name.to_lowercase())
    }

    pub fn exploration(name: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/ExplorationRank.csv", Rank, number).get(&name.to_lowercase())
    }

    pub fn trading(name: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/TradeRank.csv", Rank, number).get(&name.to_lowercase())
    }

    // New hard-coded lookups for Odyssey ranks
    pub fn exobiologist(id: &String) -> Option<&Self> {
        EXOBIOLOGIST_RANKS.get(&id.to_lowercase())
    }

    pub fn mercenary(id: &String) -> Option<&Self> {
        MERCENARY_RANKS.get(&id.to_lowercase())
    }
    
    pub fn federation(id: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/FederationRank.csv", Rank, number).get(&id.to_lowercase())
    }
    
    pub fn empire(id: &String) -> Option<&Self> {
        static_hashmap!("../fdev-ids/EmpireRank.csv", Rank, number).get(&id.to_lowercase())
    }
}

pub fn all_materials() -> state::Materials {
    let materials = static_hashmap!("../fdev-ids/material.csv", Material, symbol);
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
            locations: material_locations::component_locations(&*self.name)
                .unwrap_or_default()
                .iter()
                .map(|&s| s.to_string())
                .collect(),
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
