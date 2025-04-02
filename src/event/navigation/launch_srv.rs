use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct LaunchSRV {

    pub timestamp: String,

    #[serde(rename = "SRVType")]
    pub srvtype: String,

    #[serde(rename = "SRVType_Localised")]
    pub srvtype_localised: String,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "PlayerControlled")]
    pub player_controlled: bool,
}