use crate::text::title_case;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Materials {

    pub timestamp: String,
    
    #[serde(rename = "Raw")]
    pub raw: Vec<Material>,
    
    #[serde(rename = "Manufactured")]
    pub manufactured: Vec<Material>,
    
    #[serde(rename = "Encoded")]
    pub encoded: Vec<Material>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Material {

    #[serde(rename = "Name")]
    pub name: String,
    
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u16
}

impl Material {
    pub fn display_name(&self) -> String {
        self.name_localised.clone().unwrap_or(title_case(&self.name))
    }
}