use serde::Deserialize;
use crate::event::SuitModule;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CreateSuitLoadout {

    pub timestamp: String,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    #[serde(rename = "SuitName")]
    pub suit_name: String,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: String,

    #[serde(rename = "SuitMods")]
    pub suit_mods: Vec<String>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,

    #[serde(rename = "LoadoutName")]
    pub loadout_name: String,

    #[serde(rename = "Modules")]
    pub modules: Vec<SuitModule>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DeleteSuitLoadout {

    pub timestamp: String,

    #[serde(rename = "SuitID")]
    pub suit_id: i64,

    #[serde(rename = "SuitName")]
    pub suit_name: String,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: String,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: i64,

    #[serde(rename = "LoadoutName")]
    pub loadout_name: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SuitLoadout {

    pub timestamp: String
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SwitchSuitLoadout {

    pub timestamp: String,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    #[serde(rename = "SuitName")]
    pub suit_name: String,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: String,

    #[serde(rename = "SuitMods")]
    pub suit_mods: Vec<String>,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,

    #[serde(rename = "LoadoutName")]
    pub loadout_name: String,

    #[serde(rename = "Modules")]
    pub modules: Vec<SuitModule>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct RenameSuitLoadout {

    pub timestamp: String,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    #[serde(rename = "SuitName")]
    pub suit_name: String,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: String,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,

    #[serde(rename = "LoadoutName")]
    pub loadout_name: String,
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

    pub timestamp: String,

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