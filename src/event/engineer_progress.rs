use crate::event::engineer::Engineer;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EngineerProgress {

    pub timestamp: String,

    #[serde(rename = "Engineers")]
    pub engineers: Vec<Engineer>,
}