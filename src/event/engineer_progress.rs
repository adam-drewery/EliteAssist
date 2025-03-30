use serde::Deserialize;
use crate::event::engineer::Engineer;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EngineerProgress {

    pub timestamp: String,

    #[serde(rename = "Engineers")]
    pub engineers: Vec<Engineer>,
}