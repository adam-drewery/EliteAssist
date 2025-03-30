use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Shutdown {

    pub timestamp: String
}