use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct SuitModule {

    pub slot_name: String,
    pub suit_module_id: i64,
    pub module_name: String,
    pub module_name_localised: String,
    pub class: i64,
    pub weapon_mods: Vec<String>,
}

#[derive(Default)]
pub struct SuitLoadout {

    pub timestamp: DateTime<Utc>,
    pub suit_id: u64,
    pub suit_name: String,
    pub suit_name_localised: String,
    pub suit_mods: Vec<String>,
    pub loadout_id: u64,
    pub loadout_name: String,
    pub modules: Vec<SuitModule>,
}