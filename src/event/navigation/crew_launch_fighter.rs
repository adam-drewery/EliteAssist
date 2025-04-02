use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewLaunchFighter {

    pub timestamp: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,

    #[serde(rename = "Crew")]
    pub crew: String,
}