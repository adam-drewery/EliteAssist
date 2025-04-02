use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SynthesisMaterial {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Synthesis {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Materials")]
    pub materials: Vec<SynthesisMaterial>,
}