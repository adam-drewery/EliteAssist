use serde::Deserialize;
use crate::edsm::Counts;

#[derive(Clone, Debug, Deserialize)]
pub struct Deaths {
    pub id: Option<u64>,
    pub id64: Option<u64>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub deaths: Option<Counts>,
}