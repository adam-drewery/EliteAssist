use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct UseConsumable {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "Type")]
    pub r#type: String,
}