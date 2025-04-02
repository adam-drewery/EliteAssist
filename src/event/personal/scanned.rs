use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Scanned {

    pub timestamp: String,

    #[serde(rename = "ScanType")]
    pub scan_type: String,
}