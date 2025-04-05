use crate::state;
use crate::state::MaterialGroup;
use crate::text::title_case;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

/** These CSV files were created by copying the tables from these pages:
  https://elite-dangerous.fandom.com/wiki/Raw_Materials#List_of_Raw_Materials
  https://elite-dangerous.fandom.com/wiki/Manufactured_Materials#List_of_Manufactured_Materials
  https://elite-dangerous.fandom.com/wiki/Encoded_Materials#List_of_Encoded_Materials
*/

const ENCODED_CSV: &[u8] = include_bytes!("material/encoded.tsv");
const MANUFACTURED_CSV: &[u8] = include_bytes!("material/manufactured.tsv");
const RAW_CSV: &[u8] = include_bytes!("material/raw.tsv");

static BASE_MATERIALS: Lazy<state::Materials> = Lazy::new(|| {
    let mut materials = state::Materials {
        raw: parse_csv(RAW_CSV),
        manufactured: parse_csv(MANUFACTURED_CSV),
        encoded: parse_csv(ENCODED_CSV),
    };
    apply_category_names(&mut materials.raw);
    materials
});

#[derive(Deserialize, Debug, Default, Clone)]
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

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Material {
    
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u16,
}

#[derive(Deserialize, Debug, Default, Clone)]
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

#[derive(Deserialize, Debug, Default, Clone)]
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
            .map(|m| (m.display_name(), m.count))
            .collect();

        for group in target {
            for material in &mut group.materials {
                if let Some(&count) = count_map.get(material.name.as_str()) {
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

impl Material {
    pub fn display_name(&self) -> String {
        if let Some(ref name) = self.name_localised {
            name.to_string()
        } else {
            title_case(&self.name)
        }
    }
}

impl Into<state::Materials> for Materials {
    fn into(self) -> state::Materials {
        let mut materials = BASE_MATERIALS.clone();
        self.apply_counts(&mut materials.raw);
        self.apply_counts(&mut materials.manufactured);
        self.apply_counts(&mut materials.encoded);
        materials
    }
}

fn parse_csv(data: &[u8]) -> Vec<MaterialGroup> {
    let mut groups: HashMap<String, Vec<state::Material>> = HashMap::new();

    if let Ok(content) = std::str::from_utf8(data) {
        for line in content.lines().filter(|line| !line.is_empty()) {
            let mut parts = line.split('\t');
            let name = parts.next().unwrap_or("").trim().to_string();
            let category = parts.next().unwrap_or("").trim().to_string();
            let rarity = parts.next().unwrap_or("").trim().to_string();
            let locations = parts.next()
                .map(|s| s.trim().to_string())
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            groups
                .entry(category.clone())
                .or_default()
                .push(state::Material {
                    name,
                    count: 0,
                    locations,
                    rarity: match rarity.as_str() {
                        "Very Common" => 1,
                        "Common" => 2,
                        "Standard" => 3,
                        "Rare" => 4,
                        "Very Rare" => 5,
                        _ => 0,
                    },
                });
        }
    }

    let mut result: Vec<MaterialGroup> = groups
        .into_iter()
        .map(|(name, mut materials)| {
            materials.sort_by(|a, b| a.rarity.cmp(&b.rarity));
            MaterialGroup { name, materials }
        })
        .collect();

    result.sort_by(|a, b| a.name.cmp(&b.name));
    result
}
fn apply_category_names(material_groups: &mut [MaterialGroup]) {
    for group in material_groups {
        group.name = CATEGORY_NAMES
            .get(group.name.as_str())
            .copied()
            .unwrap_or(group.name.as_str())
            .to_string();
    }
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
