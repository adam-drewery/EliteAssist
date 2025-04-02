use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DockingCancelled {
    
    pub timestamp: String,
    
    #[serde(rename = "MarketID")]
    pub market_id: i64,
    
    #[serde(rename = "StationName")]
    pub station_name: String,
    
    #[serde(rename = "StationType")]
    pub station_type: String,
}