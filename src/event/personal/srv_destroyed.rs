use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SrvDestroyed {

    pub timestamp: String,

    #[serde(rename = "ID")]
    pub id: u64,

    #[serde(rename = "SRVType")]
    pub srvtype: String,

    #[serde(rename = "SRVType_Localised")]
    pub srvtype_localised: String,
}