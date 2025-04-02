use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct NavBeaconScan {

    pub timestamp: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "NumBodies")]
    pub num_bodies: u32,
}