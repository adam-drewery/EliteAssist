#![allow(dead_code)]

use crate::{state, lookup};
use once_cell::sync::Lazy;
use std::collections::HashMap;

// Include compile-time generated structs, data and PHF maps
include!(concat!(env!("OUT_DIR"), "/fdev_ids_gen.rs"));

impl Outfitting {
    pub fn metadata(name: &str) -> Option<&'static Self> {
        OUTFITTING_MAP
            .get(&name.to_lowercase().as_str())
            .map(|&idx| &OUTFITTING_DATA[idx])
    }
}

impl Shipyard {
    pub fn metadata(name: &str) -> Option<&'static Self> {
        SHIPYARD_MAP
            .get(&name.to_lowercase().as_str())
            .map(|&idx| &SHIPYARD_DATA[idx])
    }
}

impl Material {
    pub fn metadata(name: &str) -> Option<&'static Self> {
        MATERIAL_MAP
            .get(&name.to_lowercase().as_str())
            .map(|&idx| &MATERIAL_DATA[idx])
    }
}

impl Rank {
    pub fn cqc(name: &str) -> Option<&'static Self> {
        CQC_RANK_MAP
            .get(&name.to_lowercase().as_str())
            .map(|&idx| &CQC_RANK_DATA[idx])
    }

    pub fn combat(name: &str) -> Option<&'static Self> {
        COMBAT_RANK_MAP
            .get(&name.to_lowercase().as_str())
            .map(|&idx| &COMBAT_RANK_DATA[idx])
    }

    pub fn exploration(name: &str) -> Option<&'static Self> {
        EXPLORATION_RANK_MAP
            .get(&name.to_lowercase().as_str())
            .map(|&idx| &EXPLORATION_RANK_DATA[idx])
    }

    pub fn trading(name: &str) -> Option<&'static Self> {
        TRADE_RANK_MAP
            .get(&name.to_lowercase().as_str())
            .map(|&idx| &TRADE_RANK_DATA[idx])
    }

    // New hard-coded lookups for Odyssey ranks
    pub fn exobiologist(id: &str) -> Option<&'static Self> {
        static RANKS: Lazy<HashMap<String, Rank>> = Lazy::new(|| {
            lookup::EXOBIOLOGIST_RANKS
                .entries()
                .map(|(&k, &v)| (k.to_string(), Rank { number: k, name: v }))
                .collect()
        });
        RANKS.get(&id.to_lowercase())
    }

    pub fn mercenary(id: &str) -> Option<&'static Self> {
        static RANKS: Lazy<HashMap<String, Rank>> = Lazy::new(|| {
            lookup::MERCENARY_RANKS
                .entries()
                .map(|(&k, &v)| (k.to_string(), Rank { number: k, name: v }))
                .collect()
        });
        RANKS.get(&id.to_lowercase())
    }
    
    pub fn federation(id: &str) -> Option<&'static Self> {
        FEDERATION_RANK_MAP
            .get(&id.to_lowercase())
            .map(|&idx| &FEDERATION_RANK_DATA[idx])
    }
    
    pub fn empire(id: &str) -> Option<&'static Self> {
        EMPIRE_RANK_MAP
            .get(&id.to_lowercase())
            .map(|&idx| &EMPIRE_RANK_DATA[idx])
    }
}

pub fn all_materials() -> state::material::Materials {
    let mut raw: HashMap<&str, Vec<state::material::Material>> = HashMap::new();
    let mut encoded: HashMap<&str, Vec<state::material::Material>> = HashMap::new();
    let mut manufactured: HashMap<&str, Vec<state::material::Material>> = HashMap::new();

    for material in &MATERIAL_DATA {
        let target = match material.r#type {
            "Raw" => &mut raw,
            "Encoded" => &mut encoded,
            "Manufactured" => &mut manufactured,
            _ => continue,
        };

        target
            .entry(material.category)
            .or_insert_with(Vec::new)
            .push(material.into());
    }

    let to_sorted_groups = |map: HashMap<&str, Vec<state::material::Material>>, name_fn: fn(&str) -> &str| {
        let mut groups: Vec<_> = map
            .into_iter()
            .map(|(name, mut materials)| {
                materials.sort_by_key(|m| m.name.clone());
                state::material::Group { name: name_fn(&name).into(), materials }
            })
            .collect();
        groups.sort_by_key(|g| g.name.clone());
        groups
    };

    state::material::Materials {
        encoded: to_sorted_groups(encoded, |s| s),
        manufactured: to_sorted_groups(manufactured, |s| s),
        raw: to_sorted_groups(raw, apply_name),
    }
}

fn apply_name(input: &str) -> &str {
    lookup::CATEGORY_NAMES
        .get(input)
        .copied()
        .unwrap_or(input)
}

impl Into<state::material::Material> for &Material {
    fn into(self) -> state::material::Material {
        state::material::Material {
            count: 0,
            id: self.symbol.to_lowercase().as_str().into(),
            name: self.name.into(),
            rarity: self.rarity.parse().unwrap(),
            locations: lookup::locations_for_material(self.name)
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}