use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FetchRemoteModule {
    
    pub timestamp: String,
    
    #[serde(rename = "StorageSlot")]
    pub storage_slot: i64,
    
    #[serde(rename = "StoredItem")]
    pub stored_item: String,
    
    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localised: String,
    
    #[serde(rename = "ServerId")]
    pub server_id: i64,
    
    #[serde(rename = "TransferCost")]
    pub transfer_cost: i64,
    
    #[serde(rename = "TransferTime")]
    pub transfer_time: i64,
    
    #[serde(rename = "Ship")]
    pub ship: String,
    
    #[serde(rename = "ShipID")]
    pub ship_id: i64,
}