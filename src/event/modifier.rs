use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Modifier {

    #[serde(rename = "Label")]
    pub label: String,

    #[serde(rename = "Value")]
    pub value: f64,

    #[serde(rename = "OriginalValue")]
    pub original_value: f64,

    #[serde(rename = "LessIsGood")]
    pub less_is_good: u8,
}