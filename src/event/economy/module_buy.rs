use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleBuy {

    pub timestamp: String,

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "StoredItem")]
    pub stored_item: Option<String>,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localised: Option<String>,

    #[serde(rename = "BuyItem")]
    pub buy_item: String,

    #[serde(rename = "BuyItem_Localised")]
    pub buy_item_localised: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "BuyPrice")]
    pub buy_price: u32,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}