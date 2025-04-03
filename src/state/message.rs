use crate::event::ReceiveText;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;

pub struct Message {
    pub time: DateTime<Utc>,
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

impl From<ReceiveText> for Message {
    fn from(input: ReceiveText) -> Self {

        let (text, kind) = sanitize_name(&input.from);

        Message {
            time: input.timestamp,
            text: input.message_localised.unwrap_or(input.message),
            from: text,
            kind,
            channel: input.channel,
        }
    }
}

static NPC_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$npc_name_decorate:#name=([^;]+);$").unwrap());
static SYSTEM_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$CHAT_([^;]+);$").unwrap());
static SHIP_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$ShipName_([^;]+);$").unwrap());


fn sanitize_name(name: &String) -> (String, MessageKind) {

    if let Some(caps) = NPC_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().to_string(), MessageKind::Npc);
    }

    if let Some(caps) = SYSTEM_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().to_string(), MessageKind::System);
    }

    if let Some(caps) = SHIP_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().to_string().replace("_", ": "), MessageKind::Ship);
    }

    (name.to_string(),  MessageKind::Chat)
}