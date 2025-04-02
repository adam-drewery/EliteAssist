use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DatalinkScan {

    pub timestamp: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Message_Localised")]
    pub message_localised: String,
}