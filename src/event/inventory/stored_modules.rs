use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct StoredModule {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "StorageSlot")]
    pub storage_slot: u32,

    #[serde(rename = "StarSystem")]
    pub star_system: Option<String>,

    #[serde(rename = "MarketID")]
    pub market_id: Option<u32>,

    #[serde(rename = "TransferCost")]
    pub transfer_cost: Option<u32>,

    #[serde(rename = "TransferTime")]
    pub transfer_time: Option<u32>,

    #[serde(rename = "BuyPrice")]
    pub buy_price: u32,

    #[serde(rename = "Hot")]
    pub hot: bool,

    #[serde(rename = "EngineerModifications")]
    pub engineer_modifications: Option<String>,

    #[serde(rename = "Level")]
    pub level: Option<u32>,

    #[serde(rename = "Quality")]
    pub quality: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StoredModules {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "Items")]
    pub items: Vec<StoredModule>,
}