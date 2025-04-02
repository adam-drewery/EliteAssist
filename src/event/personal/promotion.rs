use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Promotion {

    pub timestamp: String,

    #[serde(rename = "Soldier")]
    pub soldier: Option<u32>,
}