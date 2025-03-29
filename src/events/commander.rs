use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[derive(Default)]
pub struct Commander {

    pub timestamp: String,

    #[serde(rename = "FID")]
    pub fid: String,

    #[serde(rename = "Name")]
    pub name: String
}