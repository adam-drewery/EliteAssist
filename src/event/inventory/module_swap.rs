use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ModuleSwap {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: i64,

    #[serde(rename = "FromSlot")]
    pub from_slot: String,

    #[serde(rename = "ToSlot")]
    pub to_slot: String,

    #[serde(rename = "FromItem")]
    pub from_item: String,

    #[serde(rename = "FromItem_Localised")]
    pub from_item_localised: Option<String>,

    #[serde(rename = "ToItem")]
    pub to_item: String,

    #[serde(rename = "ToItem_Localised")]
    pub to_item_localised: Option<String>,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: i64,
}