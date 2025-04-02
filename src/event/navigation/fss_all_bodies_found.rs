use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FFSSAllBodiesFound {

    pub timestamp: String,

    #[serde(rename = "SystemName")]
    pub system_name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Count")]
    pub count: u32,
}