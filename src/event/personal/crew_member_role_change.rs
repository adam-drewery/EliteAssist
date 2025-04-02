use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewMemberRoleChange {

    pub timestamp: String,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Role")]
    pub role: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}