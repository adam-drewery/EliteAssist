#[derive(Default)]
pub struct SuitModule {

    pub slot_name: String,
    pub module_name: String,
    pub class: u64,
    pub weapon_mods: Vec<String>,
}

#[derive(Default)]
pub struct SuitLoadout {

    pub suit_name: String,
    pub suit_mods: Vec<String>,
    pub loadout_name: String,
    pub modules: Vec<SuitModule>,
}