use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct LandingPadSize {

    #[serde(rename = "Small")]
    pub small: i64,

    #[serde(rename = "Medium")]
    pub medium: i64,

    #[serde(rename = "Large")]
    pub large: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DockingRequested {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: i64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "LandingPads")]
    pub landing_pads: LandingPadSize,
}