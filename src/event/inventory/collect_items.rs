use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CollectItems {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "OwnerID")]
    pub owner_id: i64,

    #[serde(rename = "Count")]
    pub count: i64,

    #[serde(rename = "Stolen")]
    pub stolen: bool,
}