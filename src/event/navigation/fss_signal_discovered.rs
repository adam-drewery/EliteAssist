use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FSSSignalDiscovered {

    pub timestamp: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "SignalName")]
    pub signal_name: String,

    #[serde(rename = "SignalType")]
    pub signal_type: String,

    #[serde(rename = "IsStation")]
    pub is_station: Option<bool>,
}