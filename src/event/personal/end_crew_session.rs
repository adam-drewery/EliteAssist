use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EndCrewSession {

    pub timestamp: String,

    #[serde(rename = "OnCrime")]
    pub on_crime: bool,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}