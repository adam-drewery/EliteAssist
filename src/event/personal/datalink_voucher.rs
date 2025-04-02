use serde::Deserialize;

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