use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SendText {

    pub timestamp: String,

    #[serde(rename = "To")]
    pub to: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Sent")]
    pub sent: bool,
}