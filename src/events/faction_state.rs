use serde::Deserialize;

#[derive(Deserialize)]
pub struct FactionState {

    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "Trend")]
    pub trend: u8,
}