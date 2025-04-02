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
pub struct HeatDamage {

    pub timestamp: String,
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
pub struct SystemsShutdown {
    pub timestamp: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SelfDestruct {

    pub timestamp: String,
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

#[derive(Deserialize, Debug, Clone)]
pub struct HeatWarning {

    pub timestamp: String,
}