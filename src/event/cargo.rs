use std::collections::HashMap;
use crate::event::*;
use crate::state;

impl Materials {

    fn apply_counts_to(&self, target: &mut Vec<MaterialGroup>) {

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
        self.apply_counts_to(&mut materials.raw);
        self.apply_counts_to(&mut materials.manufactured);
        self.apply_counts_to(&mut materials.encoded);
        materials
    }
}