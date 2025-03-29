use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Materials {

    pub timestamp: String,
    
    #[serde(rename = "Raw")]
    pub raw: Vec<Material>,
    
    #[serde(rename = "Manufactured")]
    pub manufactured: Vec<Material>,
    
    #[serde(rename = "Encoded")]
    pub encoded: Vec<Material>
}

#[derive(Deserialize, Debug)]
pub struct Material {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Count")]
    pub count: u16
}