use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartJump {

    pub timestamp: String,

    #[serde(rename = "JumpType")]
    pub jump_type: String,

    #[serde(rename = "Taxi")]
    pub taxi: bool,
}