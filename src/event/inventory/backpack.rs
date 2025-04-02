use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Backpack {

    pub timestamp: String
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BackpackItem {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "OwnerID")]
    pub owner_id: i64,

    #[serde(rename = "Count")]
    pub count: i64,

    #[serde(rename = "Type")]
    pub r#type: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BackpackChange {

    pub timestamp: String,

    #[serde(rename = "Added")]
    pub added: Option<Vec<BackpackItem>>,

    #[serde(rename = "Removed")]
    pub removed: Option<Vec<BackpackItem>>,
}