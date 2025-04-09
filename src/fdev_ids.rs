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
static SHIPYARD: &Lazy<HashMap<String, Outfitting>> = lazy_hashmap!("../FDevIDs/shipyard.csv");

pub fn read_csv<T>(bytes: &[u8]) -> HashMap<String, T>
where
    T: for<'de> Deserialize<'de> + SymbolField + std::fmt::Debug,
{
    let mut rdr = csv::Reader::from_reader(bytes);
    let mut map = HashMap::new();

    for result in rdr.deserialize() {
        let record: T = result.unwrap();

        map.insert(record.get_symbol().to_lowercase(), record);
    }

    map
}

pub trait SymbolField {
    fn get_symbol(&self) -> &str;
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

pub struct Shipyard {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub entitlement: String
}

impl SymbolField for Outfitting { fn get_symbol(&self) -> &str {
        &self.symbol
    } }
impl SymbolField for Shipyard { fn get_symbol(&self) -> &str { &self.symbol } }