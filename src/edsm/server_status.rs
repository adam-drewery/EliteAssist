use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct EliteServerStatus {

    #[serde(rename = "lastUpdate")]
    pub last_update: Box<str>,

    #[serde(rename = "type")]
    pub r#type: Box<str>,

    pub message: Box<str>,

    pub status: u8,
}