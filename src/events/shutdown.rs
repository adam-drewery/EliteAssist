use serde::Deserialize;

#[derive(Deserialize)]
pub struct Shutdown {

    pub timestamp: String
}