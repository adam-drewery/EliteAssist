use crate::fdev_ids::all_materials;
use crate::state;
use crate::state::MaterialGroup;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Materials {
    
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Raw")]
    pub raw: Vec<Material>,

    #[serde(rename = "Manufactured")]
    pub manufactured: Vec<Material>,

    #[serde(rename = "Encoded")]
    pub encoded: Vec<Material>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Material {
    
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MaterialCollected {
    
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MaterialDiscovered {
    
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "DiscoveryNumber")]
    pub discovery_number: u32,
}

impl Materials {
    fn apply_counts(&self, target: &mut Vec<MaterialGroup>) {
        let count_map: HashMap<String, u16> = self
            .raw
            .iter()
            .chain(self.manufactured.iter())
            .chain(self.encoded.iter())
            .map(|m| (m.name.clone(), m.count))
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

impl Into<state::Materials> for Materials {
    fn into(self) -> state::Materials {
        
        let mut materials = all_materials().clone();
        self.apply_counts(&mut materials.raw);
        self.apply_counts(&mut materials.manufactured);
        self.apply_counts(&mut materials.encoded);
        materials
    }
}
