use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SAAScanComplete {

    pub timestamp: String,

    #[serde(rename = "BodyName")]
    pub body_name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: i64,

    #[serde(rename = "BodyID")]
    pub body_id: i64,

    #[serde(rename = "ProbesUsed")]
    pub probes_used: i64,

    #[serde(rename = "EfficiencyTarget")]
    pub efficiency_target: i64,
}