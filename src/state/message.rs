use chrono::{DateTime, Utc};

pub struct ChatMessage {
    pub time: DateTime<Utc>,
    pub time_display: String,
    pub from: String,
    pub text: String,
    pub kind: Kind,
    pub channel: Channel,
}

pub enum Kind {
    Chat,
    System,
    Ship,
    Npc
}

pub enum Channel {
    Local,
    Npc,
    StarSystem,
    Squadron,
    SquadLeaders
}