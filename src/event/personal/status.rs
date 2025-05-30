use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Status {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Flags")]
    pub flags: u32,

    #[serde(rename = "Flags2")]
    pub flags2: Option<u32>,

    #[serde(rename = "Oxygen")]
    pub oxygen: Option<f64>,

    #[serde(rename = "Health")]
    pub health: Option<f64>,

    #[serde(rename = "Temperature")]
    pub temperature: Option<f64>,

    #[serde(rename = "SelectedWeapon")]
    pub selected_weapon: Option<String>,

    #[serde(rename = "LegalState")]
    pub legal_state: Option<String>,

    #[serde(rename = "BodyName")]
    pub body_name: Option<String>,

    #[serde(rename = "FireGroup")]
    pub fire_group: Option<u16>,

    #[serde(rename = "GuiFocus")]
    pub gui_focus: Option<u16>,

    #[serde(rename = "Cargo")]
    pub cargo: Option<f64>,

    #[serde(rename = "Fuel")]
    pub fuel: Option<Fuel>,

    #[serde(rename = "Balance")]
    pub balance: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Fuel {

    #[serde(rename = "FuelMain")]
    pub fuel_main: f64,

    #[serde(rename = "FuelReservoir")]
    pub fuel_reservoir: f64,
}