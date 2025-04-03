use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SendText {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "To")]
    pub to: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Sent")]
    pub sent: bool,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ReceiveText {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "From")]
    pub from: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Message_Localised")]
    pub message_localised: Option<String>,

    #[serde(rename = "Channel")]
    pub channel: String
}