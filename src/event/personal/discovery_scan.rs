use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DiscoveryScan {

    pub timestamp: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Bodies")]
    pub bodies: u32,
}