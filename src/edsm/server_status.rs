use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct EliteServerStatus {

    #[serde(rename = "lastUpdate")]
    pub last_update: String,

    #[serde(rename = "type")]
    pub r#type: String,

    pub message: String,

    pub status: i64,
}