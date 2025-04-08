use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;

pub fn outfitting_details(name: &String) -> Option<&'static Outfitting> {
    let name_lower = name.to_lowercase();
    OUTFITTING.get(&name_lower)
}

macro_rules! lazy_hashmap {
    ($file_path:expr) => {
        {
            const CSV_BYTES: &[u8] = include_bytes!($file_path);
            static MAP: Lazy<HashMap<String, Outfitting>> = Lazy::new(|| read_csv(CSV_BYTES));
            &MAP
        }
    };
}

static OUTFITTING: &Lazy<HashMap<String, Outfitting>> = lazy_hashmap!("../FDevIDs/outfitting.csv");

pub fn read_csv<T>(bytes: &[u8]) -> HashMap<String, T>
where
    T: for<'de> Deserialize<'de> + IdField + std::fmt::Debug,
{
    let mut rdr = csv::Reader::from_reader(bytes);
    let mut map = HashMap::new();

    for result in rdr.deserialize() {
        let record: T = result.unwrap();

        map.insert(record.get_id().to_lowercase(), record);
    }

    map
}

pub trait IdField {
    fn get_id(&self) -> &str;
}

#[derive(Deserialize)]
#[derive(Debug)]
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

impl IdField for Outfitting {
    fn get_id(&self) -> &str {
        &self.symbol
    }
}