use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Friends {
    
    pub timestamp: String,
    
    #[serde(rename = "Status")]
    pub status: String,
    
    #[serde(rename = "Name")]
    pub name: String,
}