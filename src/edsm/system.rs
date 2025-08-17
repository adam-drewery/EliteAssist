use serde::Deserialize;
use crate::edsm::Coords;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct System {
    pub name: String,
    pub url: String,
    pub coords: Coords,
    pub information: SystemInformation,
    #[serde(rename = "primaryStar")]
    pub primary_star: PrimaryStar,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SystemInformation {
    pub allegiance: String,
    pub government: String,
    pub economy: String,
    #[serde(rename = "secondEconomy")]
    pub second_economy: String,
    pub population: u64,
    pub security: String,
    pub faction: String,
    #[serde(rename = "factionState")]
    pub faction_state: String,
    pub permit: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PrimaryStar {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    #[serde(rename = "isScoopable")]
    pub is_scoopable: bool,
}