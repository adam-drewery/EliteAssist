use serde::Deserialize;
use crate::edsm::Counts;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Deaths {
    pub id: u64,
    pub id64: u64,
    pub name: String,
    pub url: String,
    pub deaths: Counts,
}