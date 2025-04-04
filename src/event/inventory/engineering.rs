use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Modifier {

    #[serde(rename = "Label")]
    pub label: String,

    #[serde(rename = "Value")]
    pub value: f64,

    #[serde(rename = "OriginalValue")]
    pub original_value: f64,

    #[serde(rename = "LessIsGood")]
    pub less_is_good: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EngineerCraft {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "Module")]
    pub module: String,

    #[serde(rename = "ApplyExperimentalEffect")]
    pub apply_experimental_effect: Option<String>,

    #[serde(rename = "Ingredients")]
    pub ingredients: Vec<super::Material>,

    #[serde(rename = "Engineer")]
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,

    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u64,

    #[serde(rename = "BlueprintName")]
    pub blueprint_name: String,

    #[serde(rename = "Level")]
    pub level: u32,

    #[serde(rename = "Quality")]
    pub quality: f64,

    #[serde(rename = "ExperimentalEffect")]
    pub experimental_effect: Option<String>,

    #[serde(rename = "ExperimentalEffect_Localised")]
    pub experimental_effect_localised: Option<String>,

    #[serde(rename = "Modifiers")]
    pub modifiers: Vec<Modifier>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EngineerContribution {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Engineer")]
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Quantity")]
    pub quantity: u32,

    #[serde(rename = "TotalQuantity")]
    pub total_quantity: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EngineerProgress {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Engineers")]
    pub engineers: Option<Vec<Engineer>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Engineer {

    #[serde(rename = "Engineer")]
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,

    #[serde(rename = "Progress")]
    pub progress: String,

    #[serde(rename = "RankProgress")]
    pub rank_progress: Option<u8>,

    #[serde(rename = "Rank")]
    pub rank: Option<u8>,
}

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