pub struct ChatMessage {
    pub time_display: String,
    pub from: String,
    pub text: String,
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