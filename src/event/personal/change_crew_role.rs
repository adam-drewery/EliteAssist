use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ChangeCrewRole {

    pub timestamp: String,

    #[serde(rename = "Role")]
    pub role: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}