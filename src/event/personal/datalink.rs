use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DatalinkScan {

    pub timestamp: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Message_Localised")]
    pub message_localised: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DatalinkVoucher {

    pub timestamp: String,

    #[serde(rename = "Reward")]
    pub reward: i64,

    #[serde(rename = "VictimFaction")]
    pub victim_faction: String,

    #[serde(rename = "PayeeFaction")]
    pub payee_faction: String,
}