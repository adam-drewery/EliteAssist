use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ClearImpound {

    pub timestamp: String,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}