use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SupercruiseEntry {

    pub timestamp: String,

    #[serde(rename = "Taxi")]
    pub taxi: bool,

    #[serde(rename = "Multicrew")]
    pub multicrew: bool,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SupercruiseExit {

    pub timestamp: String,

    #[serde(rename = "Taxi")]
    pub taxi: bool,

    #[serde(rename = "Multicrew")]
    pub multicrew: bool,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: i64,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: i64,

    #[serde(rename = "BodyType")]
    pub body_type: String,
}