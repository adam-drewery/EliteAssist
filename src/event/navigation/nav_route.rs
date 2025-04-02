use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct NavRoute {
    
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NavRouteClear {

    pub timestamp: String,
}