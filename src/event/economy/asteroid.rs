use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AsteroidCracked {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Body")]
    pub body: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AsteroidMaterial {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Proportion")]
    pub proportion: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProspectedAsteroid {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Materials")]
    pub materials: Vec<AsteroidMaterial>,

    #[serde(rename = "Content")]
    pub content: String,

    #[serde(rename = "Content_Localised")]
    pub content_localised: String,

    #[serde(rename = "Remaining")]
    pub remaining: f64,
}