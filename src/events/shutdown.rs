use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Shutdown {

    pub timestamp: String
}