use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Shipyard {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StarSystem")]
    pub star_system: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ShipyardSwap {

    pub timestamp: String,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "StoreOldShip")]
    pub store_old_ship: String,

    #[serde(rename = "StoreShipID")]
    pub store_ship_id: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ShipyardTransfer {

    pub timestamp: String,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "System")]
    pub system: String,

    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,

    #[serde(rename = "Distance")]
    pub distance: f64,

    #[serde(rename = "TransferPrice")]
    pub transfer_price: u32,

    #[serde(rename = "TransferTime")]
    pub transfer_time: u32,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}