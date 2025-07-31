use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SuitModule {
    #[serde(rename = "SlotName")]
    pub slot_name: String,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: i64,

    #[serde(rename = "ModuleName")]
    pub module_name: String,

    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localised: Option<String>,

    #[serde(rename = "Class")]
    pub class: i64,

    #[serde(rename = "WeaponMods")]
    pub weapon_mods: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SuitLoadout {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    #[serde(rename = "SuitName")]
    pub suit_name: String,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: Option<String>,

    #[serde(rename = "SuitMods")]
    pub suit_mods: Vec<String>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,

    #[serde(rename = "LoadoutName")]
    pub loadout_name: String,

    #[serde(rename = "Modules")]
    pub modules: Option<Vec<SuitModule>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SuitUpgradeResource {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize)]
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
#[derive(Clone, Debug, Deserialize)]
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
    pub suit_name_localised: Option<String>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,

    #[serde(rename = "SlotName")]
    pub slot_name: String,

    #[serde(rename = "ModuleName")]
    pub module_name: String,

    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localised: Option<String>,

    #[serde(rename = "Class")]
    pub class: u32,

    #[serde(rename = "WeaponMods")]
    pub weapon_mods: Vec<String>,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
}

impl Into<crate::state::SuitLoadout> for SuitLoadout {

    fn into(self) -> crate::state::SuitLoadout {
        crate::state::SuitLoadout {
            timestamp: self.timestamp,
            suit_id: self.suit_id,
            suit_name: self.suit_name_localised.unwrap_or_default(),
            suit_mods: self.suit_mods,
            loadout_id: self.loadout_id,
            loadout_name: self.loadout_name,
            modules: self.modules.unwrap_or_default()
                .into_iter()
                .map(|module| module.into())
                .collect(),
        }
    }
}

impl Into<crate::state::SuitModule> for SuitModule {
    fn into(self) -> crate::state::SuitModule {
        crate::state::SuitModule {
            slot_name: self.slot_name,
            suit_module_id: self.suit_module_id,
            module_name: self.module_name_localised.unwrap_or_default(),
            class: self.class,
            weapon_mods: self.weapon_mods,
        }
    }
}