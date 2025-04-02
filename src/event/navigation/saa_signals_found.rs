use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Genus {

    #[serde(rename = "Genus")]
    pub genus: String,

    #[serde(rename = "Genus_Localised")]
    pub genus_localised: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Signal {

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SAASignalsFound {

    pub timestamp: String,

    #[serde(rename = "BodyName")]
    pub body_name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: i64,

    #[serde(rename = "BodyID")]
    pub body_id: i64,

    #[serde(rename = "Signals")]
    pub signals: Vec<Signal>,

    #[serde(rename = "Genuses")]
    pub genuses: Vec<Genus>,
}