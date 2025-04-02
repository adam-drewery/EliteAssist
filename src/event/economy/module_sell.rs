use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleSell {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "SellItem")]
    pub sell_item: String,

    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localised: String,

    #[serde(rename = "SellPrice")]
    pub sell_price: u32,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}