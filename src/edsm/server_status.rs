use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct EliteServerStatus {
    pub status: EliteServerStatusInner,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct EliteServerStatusInner {
    pub message: String,
    pub status: String,
    #[serde(rename = "lastUpdate")]
    pub last_update: String,
}