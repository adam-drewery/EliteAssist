use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SearchAndRescue {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Reward")]
    pub reward: u32,
}