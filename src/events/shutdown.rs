use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Shutdown {

    pub timestamp: String
}