use crate::image::ship::*;
use phf::{Map, phf_map};

pub mod fdev_ids;

// Include compile-time generated INARA maps
include!(concat!(env!("OUT_DIR"), "/inara_gen.rs"));

pub struct SuitClass {
    pub name: &'static str,
    pub rank: u8,
}

pub static SUIT_CLASS_NAMES: Map<&'static str, SuitClass> = phf_map! {
    "explorationsuit_class1" => SuitClass {name: "Artemis", rank: 1},
    "explorationsuit_class2" => SuitClass {name: "Artemis", rank: 2},
    "explorationsuit_class3" => SuitClass {name: "Artemis", rank: 3},
    "explorationsuit_class4" => SuitClass {name: "Artemis", rank: 4},
    "explorationsuit_class5" => SuitClass {name: "Artemis", rank: 5},
    "tacticalsuit_class1" => SuitClass {name: "Dominator", rank: 1},
    "tacticalsuit_class2" => SuitClass {name: "Dominator", rank: 2},
    "tacticalsuit_class3" => SuitClass {name: "Dominator", rank: 3},
    "tacticalsuit_class4" => SuitClass {name: "Dominator", rank: 4},
    "tacticalsuit_class5" => SuitClass {name: "Dominator", rank: 5},
    "utilitysuit_class1" => SuitClass {name: "Maverick", rank: 1},
    "utilitysuit_class2" => SuitClass {name: "Maverick", rank: 2},
    "utilitysuit_class3" => SuitClass {name: "Maverick", rank: 3},
    "utilitysuit_class4" => SuitClass {name: "Maverick", rank: 4},
    "utilitysuit_class5" => SuitClass {name: "Maverick", rank: 5}
};

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
pub static SUIT_MODULE_NAMES: Map<&'static str, &'static str> = phf_map! {
    "suit_reducedtoolbatteryconsumption" => "Reduced Tool Battery Consumption",
    "suit_increasedbatterycapacity" => "Improved Battery Capacity",
    "suit_increasedshieldregen" => "Faster Shield Regen",
    "suit_improvedarmourrating" => "Damage Resistance",
    "suit_increasedo2capacity" => "Increased Air Reserves",
    "suit_nightvision" => "Night Vision",
    "suit_improvedradar" => "Enhanced Tracking",
    "suit_backpackcapacity" => "Extra Backpack Capacity",
    "suit_increasedammoreserves" => "Extra Ammo Capacity",
    "suit_improvedjumpassist" => "Improved Jump Assist",
    "suit_increasedsprintduration" => "Increased Sprint Duration",
    "suit_adsmovementspeed" => "Combat Movement Speed",
    "suit_quieterfootsteps" => "Quieter Footsteps",
    "suit_increasedmeleedamage" => "Added Melee Damage",
    "weapon_suppression_pressurised" => "Noise Suppressor",
    "weapon_suppression_unpressurised" => "Audio Masking",
    "weapon_stability" => "Stability",
    "weapon_handling" => "Faster Handling",
    "weapon_reloadspeed" => "Reload Speed",
    "weapon_clipsize" => "Magazine Size",
    "weapon_scope" => "Scope",
    "weapon_backpackreloading" => "Stowed reloading",
    "weapon_accuracy_kinematic" => "Higher Accuracy: Kinematic Armaments",
    "weapon_range_kinematic" => "Greater Range: Kinematic Armaments",
    "weapon_headshotdamage_kinematic" => "Headshot damage: Kinematic Armaments",
    "weapon_accuracy_manticore" => "Higher Accuracy: Kinematic Armaments",
    "weapon_range_manticore" => "Greater Range: Kinematic Armaments",
    "weapon_headshotdamage_manticore" => "Headshot damage: Kinematic Armaments",
    "weapon_accuracy_takada" => "Higher Accuracy: Kinematic Armaments",
    "weapon_range_takada" => "Greater Range: Kinematic Armaments",
    "weapon_headshotdamage_takada" => "Headshot damage: Kinematic Armaments"
};

pub fn locations_for_material(name: &str) -> Vec<&str> {
    get_generated_items(&MATERIAL_LOCATIONS_MAP, MATERIAL_LOCATION_LISTS, name)
}

pub fn locations_for_item(name: &str) -> Vec<&str> {
    get_generated_items(&ITEM_LOCATIONS_MAP, ITEM_LOCATION_LISTS, name)
}

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

fn get_generated_items<'a>(
    map: &Map<&'static str, usize>,
    lists: &[&[&'a str]],
    name: &str,
) -> Vec<&'a str> {
    let key = name.to_lowercase();
    if let Some(&idx) = map.get(&key) {
        if let Some(slice) = lists.get(idx) {
            return slice.iter().map(|s| *s).collect::<Vec<&str>>();
        }
    }
    Vec::new()
}
