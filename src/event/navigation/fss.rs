use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FFSSAllBodiesFound {

    pub timestamp: String,

    #[serde(rename = "SystemName")]
    pub system_name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FSSDiscoveryScan {

    pub timestamp: String,

    #[serde(rename = "Progress")]
    pub progress: f64,

    #[serde(rename = "BodyCount")]
    pub body_count: u32,

    #[serde(rename = "NonBodyCount")]
    pub non_body_count: u32,

    #[serde(rename = "SystemName")]
    pub system_name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
}

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