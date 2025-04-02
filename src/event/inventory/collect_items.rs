use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CollectItems {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Stolen")]
    pub stolen: bool,
}