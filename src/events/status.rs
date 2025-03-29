use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Status {
    pub timestamp: String,

    #[serde(rename = "Flags")]
    pub flags: i32,

    #[serde(rename = "Flags2")]
    pub flags2: i32,

    #[serde(rename = "Oxygen")]
    pub oxygen: Option<f64>,

    #[serde(rename = "Health")]
    pub health: Option<f64>,

    #[serde(rename = "Temperature")]
    pub temperature: Option<f64>,

    #[serde(rename = "SelectedWeapon")]
    pub selected_weapon: Option<String>,

    #[serde(rename = "LegalState")]
    pub legal_state: String,

    #[serde(rename = "BodyName")]
    pub body_name: Option<String>,

    #[serde(rename = "FireGroup")]
    pub fire_group: Option<i32>,

    #[serde(rename = "GuiFocus")]
    pub gui_focus: Option<i32>,

    #[serde(rename = "Cargo")]
    pub cargo: Option<f64>,

    #[serde(rename = "Fuel")]
    pub fuel: Option<Fuel>,

    #[serde(rename = "Balance")]
    pub balance: i64,
}

#[derive(Deserialize, Debug, Default)]
pub struct Fuel {

    #[serde(rename = "FuelMain")]
    pub fuel_main: f64,

    #[serde(rename = "FuelReservoir")]
    pub fuel_reservoir: f64,



}