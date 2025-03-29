use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FactionState {

    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "Trend")]
    pub trend: Option<u8>,
}