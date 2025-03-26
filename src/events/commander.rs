use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Commander {

    pub timestamp: String,

    #[serde(rename = "FID")]
    pub fid: String,

    #[serde(rename = "Name")]
    pub name: String
}