use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Disembark {

    pub timestamp: String,

    #[serde(rename = "SRV")]
    pub srv: bool,

    #[serde(rename = "Taxi")]
    pub taxi: bool,

    #[serde(rename = "Multicrew")]
    pub multicrew: bool,

    #[serde(rename = "ID")]
    pub id: u64,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "OnStation")]
    pub on_station: bool,

    #[serde(rename = "OnPlanet")]
    pub on_planet: bool,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64
}