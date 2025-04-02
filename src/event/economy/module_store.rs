use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleStore {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "StoredItem")]
    pub stored_item: String,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localised: String,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "Hot")]
    pub hot: bool,

    #[serde(rename = "EngineerModifications")]
    pub engineer_modifications: Option<String>,

    #[serde(rename = "Level")]
    pub level: Option<u32>,

    #[serde(rename = "Quality")]
    pub quality: Option<f64>,
}