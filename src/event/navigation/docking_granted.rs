use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DockingGranted {

    pub timestamp: String,

    #[serde(rename = "LandingPad")]
    pub landing_pad: u32,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,
}