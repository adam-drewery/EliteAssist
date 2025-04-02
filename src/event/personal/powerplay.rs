use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct PowerplayJoin {

    pub timestamp: String,

    #[serde(rename = "Power")]
    pub power: String,
}