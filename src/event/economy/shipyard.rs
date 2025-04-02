use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ShipyardBuy {

    pub timestamp: String,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,

    #[serde(rename = "ShipPrice")]
    pub ship_price: u32,

    #[serde(rename = "StoreOldShip")]
    pub store_old_ship: String,

    #[serde(rename = "StoreShipID")]
    pub store_ship_id: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ShipyardNew {

    pub timestamp: String,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,

    #[serde(rename = "NewShipID")]
    pub new_ship_id: u64,
}

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