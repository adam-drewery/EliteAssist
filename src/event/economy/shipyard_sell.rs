use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ShipyardSell {

    pub timestamp: String,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "SellShipID")]
    pub sell_ship_id: i64,

    #[serde(rename = "ShipPrice")]
    pub ship_price: i64,

    #[serde(rename = "MarketID")]
    pub market_id: i64,
}