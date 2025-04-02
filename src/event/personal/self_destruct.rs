use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SelfDestruct {

    pub timestamp: String,
}