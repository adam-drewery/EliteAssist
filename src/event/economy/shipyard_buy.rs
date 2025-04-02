use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ShipyardBuy {

    pub timestamp: String,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: String,

    #[serde(rename = "ShipPrice")]
    pub ship_price: i64,

    #[serde(rename = "StoreOldShip")]
    pub store_old_ship: String,

    #[serde(rename = "StoreShipID")]
    pub store_ship_id: i64,

    #[serde(rename = "MarketID")]
    pub market_id: i64,
}