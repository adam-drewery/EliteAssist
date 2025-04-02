use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ShipyardSell {

    pub timestamp: String,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "SellShipID")]
    pub sell_ship_id: u64,

    #[serde(rename = "ShipPrice")]
    pub ship_price: u32,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}