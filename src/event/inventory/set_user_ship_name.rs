use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SetUserShipName {

    pub timestamp: String,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: i64,

    #[serde(rename = "UserShipName")]
    pub user_ship_name: String,

    #[serde(rename = "UserShipId")]
    pub user_ship_id: String,
}