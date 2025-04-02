use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DiscoveryScan {

    pub timestamp: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: i64,

    #[serde(rename = "Bodies")]
    pub bodies: i64,
}