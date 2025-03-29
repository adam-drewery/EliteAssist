use serde::Deserialize;
use crate::events::fuel_capacity::FuelCapacity;
use crate::events::module::Module;

#[derive(Deserialize, Debug, Default)]
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
    pub modules: Vec<Module>
}