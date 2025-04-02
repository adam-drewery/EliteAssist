use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SuitLoadout {

    pub timestamp: String
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SuitModule {

    #[serde(rename = "SlotName")]
    pub slot_name: String,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,

    #[serde(rename = "ModuleName")]
    pub module_name: String,

    #[serde(rename = "ModuleName_Localised")]
    pub module_name_localised: String,

    #[serde(rename = "Class")]
    pub class: u32,

    #[serde(rename = "WeaponMods")]
    pub weapon_mods: Vec<String>,
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