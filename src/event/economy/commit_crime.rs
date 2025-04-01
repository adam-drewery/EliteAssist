use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CommitCrime {

    pub timestamp: String,

    #[serde(rename = "CrimeType")]
    pub crime_type: String,

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Fine")]
    pub fine: Option<i64>,
}