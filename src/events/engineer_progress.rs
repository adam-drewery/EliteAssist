use serde::Deserialize;
use crate::events::engineer::Engineer;

#[derive(Debug, Deserialize)]
pub struct EngineerProgress {

    pub timestamp: String,

    #[serde(rename = "Engineers")]
    pub engineers: Vec<Engineer>,
}