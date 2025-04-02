use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DockFighter {

    pub timestamp: String,

    #[serde(rename = "ID")]
    pub id: u64,
}