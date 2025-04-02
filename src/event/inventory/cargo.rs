use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CargoItem {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: i64,

    #[serde(rename = "Stolen")]
    pub stolen: i64,

    #[serde(rename = "MissionID")]
    pub mission_id: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cargo {

    pub timestamp: String,

    #[serde(rename = "Vessel")]
    pub vessel: String,

    #[serde(rename = "Count")]
    pub count: i64,

    #[serde(rename = "Inventory")]
    pub inventory: Option<Vec<CargoItem>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CollectCargo {

    pub timestamp: String,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: Option<String>,

    #[serde(rename = "Stolen")]
    pub stolen: bool,

    #[serde(rename = "MissionID")]
    pub mission_id: Option<i64>,
}