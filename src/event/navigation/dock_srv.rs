use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DockSRV {

    pub timestamp: String,

    #[serde(rename = "SRVType")]
    pub srvtype: String,

    #[serde(rename = "SRVType_Localised")]
    pub srvtype_localised: String,

    #[serde(rename = "ID")]
    pub id: i64,
}