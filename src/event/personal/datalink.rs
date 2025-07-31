use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DatalinkScan {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Message_Localised")]
    pub message_localised: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatalinkVoucher {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Reward")]
    pub reward: i64,

    #[serde(rename = "VictimFaction")]
    pub victim_faction: String,

    #[serde(rename = "PayeeFaction")]
    pub payee_faction: String,
}