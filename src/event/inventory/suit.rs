use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SuitModule {

    #[serde(rename = "SlotName")]
    pub slot_name: String,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: i64,

    #[serde(rename = "ModuleName")]
    pub module_name: String,

    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localised: String,

    #[serde(rename = "Class")]
    pub class: i64,

    #[serde(rename = "WeaponMods")]
    pub weapon_mods: Vec<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SuitLoadout {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    #[serde(rename = "SuitName")]
    pub suit_name: String,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: String,

    #[serde(rename = "SuitMods")]
    pub suit_mods: Option<Vec<String>>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,

    #[serde(rename = "LoadoutName")]
    pub loadout_name: String,

    #[serde(rename = "Modules")]
    pub modules: Option<Vec<SuitModule>>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SuitUpgradeResource {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct UpgradeSuit {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    #[serde(rename = "Class")]
    pub class: u32,

    #[serde(rename = "Cost")]
    pub cost: u32,

    #[serde(rename = "Resources")]
    pub resources: Vec<SuitUpgradeResource>,
}

/// Event raised when a module (suit or weapon) is added to the current loadout.
#[derive(Deserialize, Debug, Default, Clone)]
pub struct LoadoutEquipModule {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "LoadoutName")]
    pub loadout_name: String,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    #[serde(rename = "SuitName")]
    pub suit_name: String,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: String,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,

    #[serde(rename = "SlotName")]
    pub slot_name: String,

    #[serde(rename = "ModuleName")]
    pub module_name: String,

    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localised: String,

    #[serde(rename = "Class")]
    pub class: u32,

    #[serde(rename = "WeaponMods")]
    pub weapon_mods: Vec<String>,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
}