use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Embark {

    pub timestamp: String,
    
    #[serde(rename = "SRV")]
    pub srv: bool,

    #[serde(rename = "Taxi")]
    pub taxi: bool,

    #[serde(rename = "Multicrew")]
    pub multicrew: bool,

    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: i64,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: i64,

    #[serde(rename = "OnStation")]
    pub on_station: bool,

    #[serde(rename = "OnPlanet")]
    pub on_planet: bool,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: i64,
}