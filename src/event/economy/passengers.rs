use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Passenger {

    #[serde(rename = "MissionID")]
    pub mission_id: i64,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "VIP")]
    pub vip: bool,

    #[serde(rename = "Wanted")]
    pub wanted: bool,

    #[serde(rename = "Count")]
    pub count: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Passengers {

    pub timestamp: String,

    #[serde(rename = "Manifest")]
    pub manifest: Vec<Passenger>,
}