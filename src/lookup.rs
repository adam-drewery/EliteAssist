use std::collections::HashMap;
use log::warn;
use phf::{phf_map, Map};
use tokio::sync;
use crate::inara;

pub mod fdev_ids;

// Hard-coded rank maps for Exobiologist and Mercenary (Odyssey). No idea why they're not included in fdev-ids.
pub static EXOBIOLOGIST_RANKS: Map<&'static str, &'static str> = phf_map! {
    "0" => "Directionless",
    "1" => "Mostly Directionless",
    "2" => "Compiler",
    "3" => "Collector",
    "4" => "Cataloguer",
    "5" => "Taxonomist",
    "6" => "Ecologist",
    "7" => "Geneticist",
    "8" => "Elite"
};

pub static MERCENARY_RANKS: Map<&'static str, &'static str> = phf_map! {
    "0" => "Defenceless",
    "1" => "Mostly Defenceless",
    "2" => "Rookie",
    "3" => "Soldier",
    "4" => "Gunslinger",
    "5" => "Warrior",
    "6" => "Gladiator",
    "7" => "Deadeye",
    "8" => "Elite"
};

pub static CATEGORY_NAMES: Map<&'static str, &'static str> = phf_map! {
    "1" => "Light Metals and Metalloids",
    "2" => "Reactive Nonmetals and Transition Metals",
    "3" => "Chalcogens and Transition Metals",
    "4" => "Base Metals and Post-Transition Metals",
    "5" => "Coinage and Industrial Metals",
    "6" => "Heavy Metals and Metalloids",
    "7" => "Diverse Utility Elements"
};

pub fn locations_for_material(name: &str) -> Vec<String> {
    get_items(&MATERIAL_LOCATIONS, name)
}

pub fn locations_for_item(name: &str) -> Vec<String> {
    get_items(&ITEM_LOCATIONS, name)
}

// Cache of all (material, location) pairs scraped from Inara. Initialized once on first use.
static MATERIAL_LOCATIONS: sync::OnceCell<HashMap<String, Vec<String>>> = sync::OnceCell::const_new();

static ITEM_LOCATIONS: sync::OnceCell<HashMap<String, Vec<String>>> = sync::OnceCell::const_new();

fn get_items(cache: &sync::OnceCell<HashMap<String, Vec<String>>>, name: &str) -> Vec<String> {
    cache.get()
        .expect("load() called before")
        .get(&name.to_lowercase())
        .cloned()
        .unwrap_or_default()
}

pub async fn load() {
    macro_rules! init_locations {
        ($cell:expr, $method:ident) => {
            $cell
                .get_or_init(|| async {
                    let scraper = inara::Scraper::new();
                    match scraper.$method().await {
                        Ok(map) => {
                            // Convert keys to lowercase for case-insensitive lookups
                            map.into_iter()
                               .map(|(k, v)| (k.to_lowercase(), v))
                               .collect()
                        },
                        Err(e) => {
                            warn!("Inara scraping failed: {}", e);
                            return HashMap::new();
                        }
                    }
                })
                .await
        };
    }

    init_locations!(ITEM_LOCATIONS, item_locations);
    init_locations!(MATERIAL_LOCATIONS, material_locations);
}