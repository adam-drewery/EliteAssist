use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewAssign {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "CrewID")]
    pub crew_id: u64,

    #[serde(rename = "Role")]
    pub role: String,
}