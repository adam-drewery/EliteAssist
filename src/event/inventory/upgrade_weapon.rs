use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct UpgradeResource {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct UpgradeWeapon {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "Class")]
    pub class: i64,

    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: i64,

    #[serde(rename = "Cost")]
    pub cost: i64,

    #[serde(rename = "Resources")]
    pub resources: Vec<UpgradeResource>,
}