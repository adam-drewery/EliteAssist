use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoadGame {

    pub timestamp: String,

    #[serde(rename = "FID")]
    pub fid: String,

    #[serde(rename = "Commander")]
    pub commander: String,

    #[serde(rename = "Horizons")]
    pub horizons: bool,

    #[serde(rename = "Odyssey")]
    pub odyssey: bool,

    #[serde(rename = "Ship")]
    pub ship: Option<String>,

    #[serde(rename = "Ship_Localised")]
    pub ship_localised: Option<String>,

    #[serde(rename = "ShipID")]
    pub ship_id: Option<u64>,

    #[serde(rename = "ShipName")]
    pub ship_name: Option<String>,

    #[serde(rename = "ShipIdent")]
    pub ship_ident: Option<String>,

    #[serde(rename = "FuelLevel")]
    pub fuel_level: Option<f64>,

    #[serde(rename = "FuelCapacity")]
    pub fuel_capacity: Option<f64>,

    #[serde(rename = "GameMode")]
    pub game_mode: Option<String>,

    #[serde(rename = "Group")]
    pub group: Option<String>,

    #[serde(rename = "Credits")]
    pub credits: u64,

    #[serde(rename = "Loan")]
    pub loan: u64,
    pub language: String,

    #[serde(rename = "gameversion")]
    pub gameversion: String,
    pub build: String,
}