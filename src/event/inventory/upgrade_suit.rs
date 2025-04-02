use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SuitUpgradeResource {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct UpgradeSuit {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "SuitID")]
    pub suit_id: i64,

    #[serde(rename = "Class")]
    pub class: i64,

    #[serde(rename = "Cost")]
    pub cost: i64,

    #[serde(rename = "Resources")]
    pub resources: Vec<SuitUpgradeResource>,
}