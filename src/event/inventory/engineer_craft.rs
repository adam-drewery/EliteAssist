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
    pub less_is_good: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ingredient {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EngineerCraft {

    pub timestamp: String,

    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "Module")]
    pub module: String,

    #[serde(rename = "ApplyExperimentalEffect")]
    pub apply_experimental_effect: Option<String>,

    #[serde(rename = "Ingredients")]
    pub ingredients: Vec<Ingredient>,

    #[serde(rename = "Engineer")]
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: i64,

    #[serde(rename = "BlueprintID")]
    pub blueprint_id: i64,

    #[serde(rename = "BlueprintName")]
    pub blueprint_name: String,

    #[serde(rename = "Level")]
    pub level: i64,

    #[serde(rename = "Quality")]
    pub quality: f64,

    #[serde(rename = "ExperimentalEffect")]
    pub experimental_effect: String,

    #[serde(rename = "ExperimentalEffect_Localised")]
    pub experimental_effect_localised: String,

    #[serde(rename = "Modifiers")]
    pub modifiers: Vec<Modifier>,
}