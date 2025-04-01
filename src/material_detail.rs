use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::str;

#[derive(Hash, Eq, PartialEq, Debug, Default)]
pub struct MaterialDetail {
    pub name: String,
    pub category: String,
    pub rarity: String,
    pub locations: Option<String>,
}

/*
*** These CSV files were created by copying the tables from these pages:
    https://elite-dangerous.fandom.com/wiki/Raw_Materials#List_of_Raw_Materials
    https://elite-dangerous.fandom.com/wiki/Manufactured_Materials#List_of_Manufactured_Materials
    https://elite-dangerous.fandom.com/wiki/Encoded_Materials#List_of_Encoded_Materials
 */

const ENCODED_CSV: &[u8] = include_bytes!("material_detail/encoded.tsv");
const MANUFACTURED_CSV: &[u8] = include_bytes!("material_detail/manufactured.tsv");
const RAW_CSV: &[u8] = include_bytes!("material_detail/raw.tsv");

fn parse_csv(data: &[u8]) -> HashSet<MaterialDetail> {
    str::from_utf8(data)
        .unwrap_or_default()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split('\t');
            MaterialDetail {
                name: parts.next().unwrap_or("").trim().to_string(),
                category: parts.next().unwrap_or("").trim().to_string(),
                rarity: parts.next().unwrap_or("").trim().to_string(),
                locations: parts.next().map(|s| s.trim().to_string()),
            }
        })
        .collect()
}

fn get_all_materials() -> HashSet<MaterialDetail> {
    let mut all_materials = HashSet::new();
    all_materials.extend(parse_csv(ENCODED_CSV));
    all_materials.extend(parse_csv(MANUFACTURED_CSV));
    all_materials.extend(parse_csv(RAW_CSV));
    all_materials
}

pub fn find_material<'a>(name: &String) -> Option<&'a MaterialDetail> {
    static MATERIALS: Lazy<HashSet<MaterialDetail>> = Lazy::new(get_all_materials);
    MATERIALS.iter().find(|m| m.name == *name)
}