use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrimeVictim {

    pub timestamp: String,

    #[serde(rename = "Offender")]
    pub offender: String,

    #[serde(rename = "CrimeType")]
    pub crime_type: String,

    #[serde(rename = "Fine")]
    pub fine: Option<u32>,
}