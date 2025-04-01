use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DockingDenied {

    pub timestamp: String,

    #[serde(rename = "Reason")]
    pub reason: String,

    #[serde(rename = "MarketID")]
    pub market_id: i64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,
}