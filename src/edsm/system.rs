use serde::Deserialize;
use crate::edsm::Coords;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct System {
    pub id: i64,
    pub id64: i64,
    pub name: String,
    pub coords: Coords,
    #[serde(rename = "coordsLocked")]
    pub coords_locked: bool,
    pub information: SystemInformation,
    #[serde(rename = "primaryStar")]
    pub primary_star: PrimaryStar,
    #[serde(rename = "requirePermit")]
    pub require_permit: bool,
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
    pub faction_state: String
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PrimaryStar {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    #[serde(rename = "isScoopable")]
    pub is_scoopable: bool,
}