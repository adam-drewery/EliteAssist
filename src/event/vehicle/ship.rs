use crate::event::Engineering;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SetUserShipName {

    pub timestamp: String,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "UserShipName")]
    pub user_ship_name: String,

    #[serde(rename = "UserShipId")]
    pub user_ship_id: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct AfmuRepairs {

    pub timestamp: String,

    #[serde(rename = "Module")]
    pub module: String,

    #[serde(rename = "Module_Localised")]
    pub module_localised: String,

    #[serde(rename = "FullyRepaired")]
    pub fully_repaired: bool,

    #[serde(rename = "Health")]
    pub health: f64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct HullDamage {

    pub timestamp: String,

    #[serde(rename = "Health")]
    pub health: f64,

    #[serde(rename = "PlayerPilot")]
    pub player_pilot: bool,

    #[serde(rename = "Fighter")]
    pub fighter: Option<bool>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Ship {

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "Name")]
    pub name: Option<String>,

    #[serde(rename = "Value")]
    pub value: u32,

    #[serde(rename = "Hot")]
    pub hot: bool,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct StoredShips {

    pub timestamp: String,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "ShipsHere")]
    pub ships_here: Vec<Ship>,

    #[serde(rename = "ShipsRemote")]
    pub ships_remote: Vec<Ship>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ShipLoadout {

    pub timestamp: String,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "ShipName")]
    pub ship_name: String,

    #[serde(rename = "ShipIdent")]
    pub ship_ident: String,

    #[serde(rename = "HullValue")]
    pub hull_value: u64,

    #[serde(rename = "ModulesValue")]
    pub modules_value: u64,

    #[serde(rename = "HullHealth")]
    pub hull_health: f64,

    #[serde(rename = "UnladenMass")]
    pub unladen_mass: f64,

    #[serde(rename = "CargoCapacity")]
    pub cargo_capacity: u64,

    #[serde(rename = "MaxJumpRange")]
    pub max_jump_range: f64,

    #[serde(rename = "FuelCapacity")]
    pub fuel_capacity: FuelCapacity,

    #[serde(rename = "Rebuy")]
    pub rebuy: u64,

    #[serde(rename = "Modules")]
    pub modules: Vec<ShipModule>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ShipModule {

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "Item")]
    pub item: String,

    #[serde(rename = "On")]
    pub on: bool,

    #[serde(rename = "Priority")]
    pub priority: u8,

    #[serde(rename = "Health")]
    pub health: f64,

    #[serde(rename = "Value")]
    pub value: Option<u64>,

    #[serde(rename = "AmmoInClip")]
    pub ammo_in_clip: Option<u64>,

    #[serde(rename = "AmmoInHopper")]
    pub ammo_in_hopper: Option<u64>,

    #[serde(rename = "Engineering")]
    pub engineering: Option<Engineering>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FuelCapacity {

    #[serde(rename = "Main")]
    pub main: f64,

    #[serde(rename = "Reserve")]
    pub reserve: f64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct LoadoutEquipModule {

    pub timestamp: String,

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