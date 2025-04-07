use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct BuyWeapon {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Class")]
    pub class: u32,

    #[serde(rename = "Price")]
    pub price: u32,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,

    #[serde(rename = "WeaponMods")]
    pub weapon_mods: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SellWeapon {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Class")]
    pub class: u32,

    #[serde(rename = "WeaponMods")]
    pub weapon_mods: Vec<String>,

    #[serde(rename = "Price")]
    pub price: u32,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
}