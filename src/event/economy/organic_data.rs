use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BioData {

    #[serde(rename = "Genus")]
    pub genus: String,

    #[serde(rename = "Genus_Localised")]
    pub genus_localised: String,

    #[serde(rename = "Species")]
    pub species: String,

    #[serde(rename = "Species_Localised")]
    pub species_localised: String,

    #[serde(rename = "Variant")]
    pub variant: String,

    #[serde(rename = "Variant_Localised")]
    pub variant_localised: String,

    #[serde(rename = "Value")]
    pub value: u32,

    #[serde(rename = "Bonus")]
    pub bonus: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SellOrganicData {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "BioData")]
    pub bio_data: Vec<BioData>,
}