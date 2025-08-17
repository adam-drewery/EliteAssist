use serde::Deserialize;
use crate::edsm::Counts;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Traffic {
    pub id: u64,
    pub id64: u64,
    pub name: String,
    pub url: String,
    pub traffic: Counts,
}