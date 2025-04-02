use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SAAScanComplete {

    pub timestamp: String,

    #[serde(rename = "BodyName")]
    pub body_name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "ProbesUsed")]
    pub probes_used: u32,

    #[serde(rename = "EfficiencyTarget")]
    pub efficiency_target: u32,
}