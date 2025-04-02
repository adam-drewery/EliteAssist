use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct AsteroidCracked {

    pub timestamp: String,

    #[serde(rename = "Body")]
    pub body: String,
}