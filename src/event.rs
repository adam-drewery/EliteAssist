use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "event")] // instruct Serde that "event" determines the variant
pub enum EliteEvent {
    Docked {
        event_type: String,
        timestamp: String,
        StationName: String
    },
    FSDJump {
        event_type: String,
        timestamp: String,
        StarSystem: String,
    }
}