use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ShieldState {

    pub timestamp: String,

    #[serde(rename = "ShieldsUp")]
    pub shields_up: bool,
}