#![allow(dead_code)]

use crate::{state, lookup};
use once_cell::sync::Lazy;
use std::collections::HashMap;

// Include compile-time generated structs, data and PHF maps
include!(concat!(env!("OUT_DIR"), "/fdev_ids_gen.rs"));

impl Outfitting {
    pub fn metadata(name: &String) -> Option<&'static Self> {
        OUTFITTING_MAP
            .get(&name.to_lowercase())
            .map(|&idx| &OUTFITTING_DATA[idx])
    }
}

impl Shipyard {
    pub fn metadata(name: &String) -> Option<&'static Self> {
        SHIPYARD_MAP
            .get(&name.to_lowercase())
            .map(|&idx| &SHIPYARD_DATA[idx])
    }
}

impl Material {
    pub fn metadata(name: &String) -> Option<&'static Self> {
        MATERIAL_MAP
            .get(&name.to_lowercase())
            .map(|&idx| &MATERIAL_DATA[idx])
    }
}

impl Rank {
    pub fn cqc(name: &String) -> Option<&'static Self> {
        CQC_RANK_MAP
            .get(&name.to_lowercase())
            .map(|&idx| &CQC_RANK_DATA[idx])
    }

    pub fn combat(name: &String) -> Option<&'static Self> {
        COMBAT_RANK_MAP
            .get(&name.to_lowercase())
            .map(|&idx| &COMBAT_RANK_DATA[idx])
    }

    pub fn exploration(name: &String) -> Option<&'static Self> {
        EXPLORATION_RANK_MAP
            .get(&name.to_lowercase())
            .map(|&idx| &EXPLORATION_RANK_DATA[idx])
    }

    pub fn trading(name: &String) -> Option<&'static Self> {
        TRADE_RANK_MAP
            .get(&name.to_lowercase())
            .map(|&idx| &TRADE_RANK_DATA[idx])
    }

    // New hard-coded lookups for Odyssey ranks
    pub fn exobiologist(id: &String) -> Option<&'static Self> {
        static RANKS: Lazy<HashMap<String, Rank>> = Lazy::new(|| {
            lookup::EXOBIOLOGIST_RANKS
                .entries()
                .map(|(&k, &v)| (k.to_string(), Rank { number: k, name: v }))
                .collect()
        });
        RANKS.get(&id.to_lowercase())
    }

    pub fn mercenary(id: &String) -> Option<&'static Self> {
        static RANKS: Lazy<HashMap<String, Rank>> = Lazy::new(|| {
            lookup::MERCENARY_RANKS
                .entries()
                .map(|(&k, &v)| (k.to_string(), Rank { number: k, name: v }))
                .collect()
        });
        RANKS.get(&id.to_lowercase())
    }
    
    pub fn federation(id: &String) -> Option<&'static Self> {
        FEDERATION_RANK_MAP
            .get(&id.to_lowercase())
            .map(|&idx| &FEDERATION_RANK_DATA[idx])
    }
    
    pub fn empire(id: &String) -> Option<&'static Self> {
        EMPIRE_RANK_MAP
            .get(&id.to_lowercase())
            .map(|&idx| &EMPIRE_RANK_DATA[idx])
    }
}

pub fn all_materials() -> state::Materials {
    let mut raw: HashMap<String, Vec<state::Material>> = HashMap::new();
    let mut encoded: HashMap<String, Vec<state::Material>> = HashMap::new();
    let mut manufactured: HashMap<String, Vec<state::Material>> = HashMap::new();

    for material in &MATERIAL_DATA {
        let target = match material.r#type {
            "Raw" => &mut raw,
            "Encoded" => &mut encoded,
            "Manufactured" => &mut manufactured,
            _ => continue,
        };

        target
            .entry(material.category.to_string())
            .or_insert_with(Vec::new)
            .push(material.into());
    }

    let to_sorted_groups = |map: HashMap<String, Vec<state::Material>>, name_fn: fn(&str) -> String| {
        let mut groups: Vec<_> = map
            .into_iter()
            .map(|(name, mut materials)| {
                materials.sort_by_key(|m| m.name.clone());
                state::MaterialGroup { name: name_fn(&name), materials }
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

fn apply_name(input: &str) -> String {
    lookup::CATEGORY_NAMES
        .get(input)
        .copied()
        .unwrap_or(input)
        .to_string()
}

impl Into<state::Material> for &Material {
    fn into(self) -> state::Material {
        state::Material {
            id: self.symbol.to_lowercase(),
            name: self.name.to_string(),
            rarity: self.rarity.parse().unwrap(),
            count: 0,
            locations: lookup::locations_for_material(self.name),
        }
    }
}