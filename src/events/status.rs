use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Status {
    pub timestamp: String,

    #[serde(rename = "Flags")]
    pub flags: i32,

    #[serde(rename = "Flags2")]
    pub flags2: i32,

    #[serde(rename = "Oxygen")]
    pub oxygen: f64,

    #[serde(rename = "Health")]
    pub health: f64,

    #[serde(rename = "Temperature")]
    pub temperature: f64,

    #[serde(rename = "SelectedWeapon")]
    pub selected_weapon: String,

    #[serde(rename = "LegalState")]
    pub legal_state: String,

    #[serde(rename = "BodyName")]
    pub body_name: String,

    #[serde(rename = "Balance")]
    pub balance: i64,
}