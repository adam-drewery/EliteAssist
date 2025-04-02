use serde::Deserialize;

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