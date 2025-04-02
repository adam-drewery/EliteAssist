use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewMemberJoins {

    pub timestamp: String,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}