use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MiningRefined {

    pub timestamp: String,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: String,
}