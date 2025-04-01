use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct UnderAttack {

    pub timestamp: String,

    #[serde(rename = "Target")]
    pub target: String,
}