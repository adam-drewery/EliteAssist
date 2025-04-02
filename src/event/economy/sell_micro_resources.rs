use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MicroResource {

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
pub struct SellMicroResources {

    pub timestamp: String,

    #[serde(rename = "TotalCount")]
    pub total_count: u32,

    #[serde(rename = "MicroResources")]
    pub micro_resources: Vec<MicroResource>,

    #[serde(rename = "Price")]
    pub price: u32,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}