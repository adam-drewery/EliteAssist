use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MaterialDiscovered {

    pub timestamp: String,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "DiscoveryNumber")]
    pub discovery_number: i64,
}