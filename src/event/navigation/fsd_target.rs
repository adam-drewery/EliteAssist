use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FSDTarget {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "StarClass")]
    pub star_class: String,

    #[serde(rename = "RemainingJumpsInRoute")]
    pub remaining_jumps_in_route: Option<u32>,
}