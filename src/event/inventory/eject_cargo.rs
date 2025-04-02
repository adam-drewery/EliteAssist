use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EjectCargo {

    pub timestamp: String,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "Abandoned")]
    pub abandoned: bool,
}