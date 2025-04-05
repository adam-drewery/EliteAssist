use chrono::{DateTime, Utc};

pub struct ChatMessage {
    pub time: DateTime<Utc>,
    pub time_display: String,
    pub from: String,
    pub text: String,
    pub kind: MessageKind,
    pub channel: String,
}

pub enum MessageKind {
    Chat,
    System,
    Ship,
    Npc
}