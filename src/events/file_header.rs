use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct FileHeader {

    pub timestamp: String,
    
    pub part: u8,
    
    pub language: String,

    #[serde(rename = "gameversion")]
    pub game_version: String,
    
    pub build: String
}