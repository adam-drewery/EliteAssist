use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleSell {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: i64,

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "SellItem")]
    pub sell_item: String,

    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localised: String,

    #[serde(rename = "SellPrice")]
    pub sell_price: i64,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: i64,
}