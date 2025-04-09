use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct SystemExplorationData {

    #[serde(rename = "SystemName")]
    pub system_name: String,

    #[serde(rename = "NumBodies")]
    pub num_bodies: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MultiSellExplorationData {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Discovered")]
    pub discovered: Vec<SystemExplorationData>,

    #[serde(rename = "BaseValue")]
    pub base_value: u32,

    #[serde(rename = "Bonus")]
    pub bonus: u32,

    #[serde(rename = "TotalEarnings")]
    pub total_earnings: u32,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct SellOrganicData {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "BioData")]
    pub bio_data: Vec<BioData>,
}