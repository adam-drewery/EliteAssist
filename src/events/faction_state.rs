use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct FactionState {

    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "Trend")]
    pub trend: Option<u8>,
}