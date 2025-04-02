use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewMemberQuits {

    pub timestamp: String,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}