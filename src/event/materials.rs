use once_cell::sync::Lazy;
use crate::text::title_case;
use serde::Deserialize;
use crate::material_detail::{find_material, MaterialDetail};

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Materials {

    pub timestamp: String,

    #[serde(rename = "Raw")]
    pub raw: Vec<Material>,

    #[serde(rename = "Manufactured")]
    pub manufactured: Vec<Material>,

    #[serde(rename = "Encoded")]
    pub encoded: Vec<Material>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Material {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u16,
}

impl Material {
    
    pub fn display_name(&self) -> String {
        self.name_localised
            .as_ref()
            .map(|name| name.clone())
            .unwrap_or_else(|| title_case(&self.name))
    }
    pub fn info(&self) -> &MaterialDetail {

        static DEFAULT_MATERIAL: Lazy<MaterialDetail> = Lazy::new(|| MaterialDetail::default());
        let display_name = self.display_name();
        
        match find_material(&display_name) {
            Some(material) => material,
            None => { 
                eprintln!("Material not found: {}", display_name);
                &*DEFAULT_MATERIAL
            }
        }
    }
}
