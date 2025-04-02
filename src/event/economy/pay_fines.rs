use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct PayFines {

    pub timestamp: String,

    #[serde(rename = "Amount")]
    pub amount: i64,

    #[serde(rename = "AllFines")]
    pub all_fines: bool,

    #[serde(rename = "ShipID")]
    pub ship_id: i64,
}