use crate::lookup::fdev_ids::all_materials;
use std::collections::HashMap;
use ed_journals::logs::materials_event::MaterialsEvent;

#[derive(Clone, Default)]
pub struct Materials {
    pub raw: Vec<MaterialGroup>,
    pub manufactured: Vec<MaterialGroup>,
    pub encoded: Vec<MaterialGroup>,
}

#[derive(Clone)]
pub struct MaterialGroup {
    pub name: String,
    pub materials: Vec<Material>,
}

#[derive(Clone)]
pub struct Material {
    pub id: String,
    pub name: String,
    pub rarity: u8,
    pub count: u16,
    pub locations: Vec<String>,
}

impl From<MaterialsEvent> for Materials {
    fn from(value: MaterialsEvent) -> Self {
        // Build a name->count map from the event
        let count_map: HashMap<String, u16> = value
            .raw
            .iter()
            .map(|m| (m.name.to_string(), m.count))
            .chain(value.manufactured.iter().map(|m| (m.name.to_string(), m.count)))
            .chain(value.encoded.iter().map(|m| (m.name.to_string(), m.count)))
            .collect();

        // Start with the canonical materials list, then apply counts where present
        let mut materials = all_materials().clone();
        for group in &mut materials.raw {
            for material in &mut group.materials {
                material.count = *count_map.get(&material.id).unwrap_or(&0);
            }
        }
        for group in &mut materials.manufactured {
            for material in &mut group.materials {
                material.count = *count_map.get(&material.id).unwrap_or(&0);
            }
        }
        for group in &mut materials.encoded {
            for material in &mut group.materials {
                material.count = *count_map.get(&material.id).unwrap_or(&0);
            }
        }
        materials
    }
}
