use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReceiveText {

    pub timestamp: String,

    #[serde(rename = "From")]
    pub from: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Message_Localised")]
    pub message_localised: String,

    #[serde(rename = "Channel")]
    pub channel: String
}