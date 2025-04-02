use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleSellRemote {

    pub timestamp: String,

    #[serde(rename = "StorageSlot")]
    pub storage_slot: u32,

    #[serde(rename = "SellItem")]
    pub sell_item: String,

    #[serde(rename = "SellItem_Localised")]
    pub sell_item_localised: String,

    #[serde(rename = "ServerId")]
    pub server_id: u64,

    #[serde(rename = "SellPrice")]
    pub sell_price: u32,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}