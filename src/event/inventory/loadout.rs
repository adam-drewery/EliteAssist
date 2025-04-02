use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Loadout {

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
    pub modules: Vec<LoadoutModule>
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoadoutModule {

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

#[derive(Deserialize, Debug, Clone)]
pub struct Engineering {

    #[serde(rename = "Engineer")]
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,

    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u64,

    #[serde(rename = "BlueprintName")]
    pub blueprint_name: String,

    #[serde(rename = "Level")]
    pub level: u8,

    #[serde(rename = "Quality")]
    pub quality: f64,

    #[serde(rename = "ExperimentalEffect")]
    pub experimental_effect: Option<String>,

    #[serde(rename = "ExperimentalEffect_Localised")]
    pub experimental_effect_localised: Option<String>,

    #[serde(rename = "Modifiers")]
    pub modifiers: Vec<Modifier>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Modifier {

    #[serde(rename = "Label")]
    pub label: String,

    #[serde(rename = "Value")]
    pub value: f64,

    #[serde(rename = "OriginalValue")]
    pub original_value: f64,

    #[serde(rename = "LessIsGood")]
    pub less_is_good: u8,
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