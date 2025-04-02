use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CodexEntry {

    pub timestamp: String,

    #[serde(rename = "EntryID")]
    pub entry_id: u64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "SubCategory")]
    pub sub_category: String,

    #[serde(rename = "SubCategory_Localised")]
    pub sub_category_localised: String,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Category_Localised")]
    pub category_localised: String,

    #[serde(rename = "Region")]
    pub region: String,

    #[serde(rename = "Region_Localised")]
    pub region_localised: String,

    #[serde(rename = "System")]
    pub system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "Latitude")]
    pub latitude: Option<f64>,

    #[serde(rename = "Longitude")]
    pub longitude: Option<f64>,

    #[serde(rename = "IsNewEntry")]
    pub is_new_entry: Option<bool>,
}