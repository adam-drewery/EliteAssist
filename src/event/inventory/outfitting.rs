use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Outfitting {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: i64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StarSystem")]
    pub star_system: String,
}