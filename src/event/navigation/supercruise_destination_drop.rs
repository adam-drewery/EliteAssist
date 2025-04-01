use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SupercruiseDestinationDrop {

    pub timestamp: String,


    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Threat")]
    pub threat: i64,
}