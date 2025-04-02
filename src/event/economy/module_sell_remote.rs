use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleSellRemote {

    pub timestamp: String,

    #[serde(rename = "StorageSlot")]
    pub storage_slot: i64,

    #[serde(rename = "SellItem")]
    pub sell_item: String,

    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localised: String,

    #[serde(rename = "ServerId")]
    pub server_id: i64,

    #[serde(rename = "SellPrice")]
    pub sell_price: i64,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: i64,
}