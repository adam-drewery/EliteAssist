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

#[derive(Debug, Deserialize)]
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

impl Outfitting {
    pub fn metadata(name: &String) -> Option<&Self> {
        static_hashmap!("../FDevIDs/outfitting.csv", Outfitting).get(&name.to_lowercase())
    }
}

impl Shipyard {
    pub fn metadata(name: &String) -> Option<&Self> {
        static_hashmap!("../FDevIDs/shipyard.csv", Shipyard).get(&name.to_lowercase())
    }
}