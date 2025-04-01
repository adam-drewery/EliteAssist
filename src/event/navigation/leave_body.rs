use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LeaveBody {

    pub timestamp: String,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: i64,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: i64,
}