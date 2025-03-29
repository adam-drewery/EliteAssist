use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ShipLockerData {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    #[serde(rename = "MissionID")]
    pub mission_id: Option<u64>,

    #[serde(rename = "Count")]
    pub count: u64
}