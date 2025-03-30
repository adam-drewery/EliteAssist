use serde::Deserialize;
use crate::event::modifier::Modifier;

#[derive(Deserialize, Debug, Clone)]
pub struct Engineering {

    #[serde(rename = "Engineer")]
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,

    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u64,

    #[serde(rename = "BlueprintName")]
    pub blueprint_name: String,

    #[serde(rename = "Level")]
    pub level: u8,

    #[serde(rename = "Quality")]
    pub quality: f64,

    #[serde(rename = "ExperimentalEffect")]
    pub experimental_effect: Option<String>,

    #[serde(rename = "ExperimentalEffect_Localised")]
    pub experimental_effect_localised: Option<String>,

    #[serde(rename = "Modifiers")]
    pub modifiers: Vec<Modifier>,
}