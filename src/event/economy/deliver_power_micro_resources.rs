use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MicroResource {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Count")]
    pub count: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DeliverPowerMicroResources {

    pub timestamp: String,

    #[serde(rename = "TotalCount")]
    pub total_count: i64,

    #[serde(rename = "MicroResources")]
    pub micro_resources: Vec<MicroResource>,

    #[serde(rename = "MarketID")]
    pub market_id: i64,
}