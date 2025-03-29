use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Shutdown {

    pub timestamp: String
}