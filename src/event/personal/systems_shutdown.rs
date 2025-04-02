use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SystemsShutdown {
    pub timestamp: String,
}