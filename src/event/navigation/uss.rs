use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct USSDrop {

    pub timestamp: String,

    #[serde(rename = "USSType")]
    pub usstype: String,

    #[serde(rename = "USSType_Localised")]
    pub usstype_localised: String,

    #[serde(rename = "USSThreat")]
    pub ussthreat: u32,
}