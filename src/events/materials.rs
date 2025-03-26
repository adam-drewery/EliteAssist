use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Materials {

    pub timestamp: String,
    pub materials: Vec<Material>
}

#[derive(Debug, Deserialize)]
pub struct Material {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Count")]
    pub count: u8
}