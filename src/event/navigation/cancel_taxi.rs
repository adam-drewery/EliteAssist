use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CancelTaxi {

    pub timestamp: String,

    #[serde(rename = "Refund")]
    pub refund: u32,
}