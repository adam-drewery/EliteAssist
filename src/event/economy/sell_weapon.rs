use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SellWeapon {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "Class")]
    pub class: u32,

    #[serde(rename = "WeaponMods")]
    pub weapon_mods: Vec<String>,

    #[serde(rename = "Price")]
    pub price: u32,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
}