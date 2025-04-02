use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct VehicleSwitch {

    pub timestamp: String,

    #[serde(rename = "To")]
    pub to: String,
}