use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Reputation {

    pub timestamp: String,

    #[serde(rename = "Empire")]
    pub empire: f64,

    #[serde(rename = "Federation")]
    pub federation: f64,

    #[serde(rename = "Independent")]
    pub independent: f64,

    #[serde(rename = "Alliance")]
    pub alliance: f64
}