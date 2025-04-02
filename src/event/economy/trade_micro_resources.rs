use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct OfferedMicroResource {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct TradeMicroResources {

    pub timestamp: String,

    #[serde(rename = "Offered")]
    pub offered: Vec<OfferedMicroResource>,

    #[serde(rename = "TotalCount")]
    pub total_count: u32,

    #[serde(rename = "Received")]
    pub received: String,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}