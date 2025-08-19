use std::collections::HashMap;
use log::warn;
use phf::{phf_map, Map};
use tokio::sync;
use crate::inara;
use crate::image::ship::*;

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

// Ship image bytes lookup: maps ship names and common aliases to image bytes (exact match)
pub static SHIP_IMAGES: Map<&'static str, &'static [u8]> = phf_map! {
    // Core names (as displayed)
    "Adder" => ADDER_PNG,
    "Alliance Challenger" => ALLIANCE_CHALLENGER_PNG,
    "Alliance Chieftain" => ALLIANCE_CHIEFTAIN_PNG,
    "Alliance Crusader" => ALLIANCE_CRUSADER_PNG,
    "Anaconda" => ANACONDA_PNG,
    "Asp Explorer" => ASP_EXPLORER_PNG,
    "Asp Scout" => ASP_SCOUT_PNG,
    "Beluga Liner" => BELUGA_LINER_PNG,
    "Cobra Mk III" => COBRA_MK_III_PNG,
    "Cobra Mk IV" => COBRA_MK_IV_PNG,
    "Cobra Mk V" => COBRA_MK_V_PNG,
    "Corsair" => CORSAIR_PNG,
    "Cyclops" => CYCLOPS_PNG,
    "Diamondback Explorer" => DIAMONDBACK_EXPLORER_PNG,
    "Diamondback Scout" => DIAMONDBACK_SCOUT_PNG,
    "Dolphin" => DOLPHIN_PNG,
    "Eagle Mk II" => EAGLE_MK_II_PNG,
    "Federal Assault Ship" => FEDERAL_ASSAULT_SHIP_PNG,
    "Federal Corvette" => FEDERAL_CORVETTE_PNG,
    "Federal Dropship" => FEDERAL_DROPSHIP_PNG,
    "Federal Gunship" => FEDERAL_GUNSHIP_PNG,
    "Hauler" => HAULER_PNG,
    "Imperial Clipper" => IMPERIAL_CLIPPER_PNG,
    "Imperial Courier" => IMPERIAL_COURIER_PNG,
    "Imperial Cutter" => IMPERIAL_CUTTER_PNG,
    "Imperial Eagle" => IMPERIAL_EAGLE_PNG,
    "Keelback" => KEELBACK_PNG,
    "Krait Mk II" => KRAIT_MK_II_PNG,
    "Krait Phantom" => KRAIT_PHANTOM_PNG,
    "Mamba" => MAMBA_PNG,
    "Mandalay" => MANDALAY_PNG,
    "Orca" => ORCA_PNG,
    "Panther Clipper Mk II" => PANTHER_CLIPPER_MK_II_PNG,
    "Python Mk II" => PYTHON_MK_II_PNG,
    "Python" => PYTHON_PNG,
    "Sidewinder" => SIDEWINDER_PNG,
    "Type 10 Defender" => TYPE_10_DEFENDER_PNG,
    "Type-11 Prospector" => TYPE_11_PROSPECTOR_PNG,
    "Type 6 Transporter" => TYPE_6_TRANSPORTER_PNG,
    "Type 7 Transporter" => TYPE_7_TRANSPORTER_PNG,
    "Type-8 Transporter" => TYPE_8_TRANSPORTER_PNG,
    "Type 9 Heavy" => TYPE_9_HEAVY_PNG,
    "Viper Mk III" => VIPER_MK_III_PNG,
    "Viper Mk IV" => VIPER_MK_IV_PNG,
    "Vulture" => VULTURE_PNG,

    // Common aliases
    // Mk with no space
    "Cobra MkIII" => COBRA_MK_III_PNG,
    "Cobra MkIV" => COBRA_MK_IV_PNG,
    "Cobra MkV" => COBRA_MK_V_PNG,
    "Eagle MkII" => EAGLE_MK_II_PNG,
    "Krait MkII" => KRAIT_MK_II_PNG,
    "Panther Clipper MkII" => PANTHER_CLIPPER_MK_II_PNG,
    "Python MkII" => PYTHON_MK_II_PNG,
    "Viper MkIII" => VIPER_MK_III_PNG,
    "Viper MkIV" => VIPER_MK_IV_PNG,

    // Hyphen/space variants
    "Type-10 Defender" => TYPE_10_DEFENDER_PNG,
    "Type 11 Prospector" => TYPE_11_PROSPECTOR_PNG,
    "Type-6 Transporter" => TYPE_6_TRANSPORTER_PNG,
    "Type-7 Transporter" => TYPE_7_TRANSPORTER_PNG,
    "Type 8 Transporter" => TYPE_8_TRANSPORTER_PNG,

    // Fer-de-Lance variant
    "Fer-de-Lance" => FER_DE_LANCE_PNG,
};

pub fn ship_image_bytes(name: &str) -> Option<&'static [u8]> {
    SHIP_IMAGES.get(name).copied()
}
