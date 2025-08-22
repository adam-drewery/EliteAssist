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

use crate::journal::event;
use crate::journal::format::prettify_date;
use once_cell::sync::Lazy;
use regex::Regex;

impl From<event::ReceiveText> for ChatMessage {
    fn from(value: event::ReceiveText) -> Self {
        let (text, _kind) = sanitize_name(&value.from);
        ChatMessage {
            time_display: prettify_date(&value.timestamp),
            text: value.message_localised.unwrap_or(value.message),
            from: text,
            channel: match value.channel.as_str() {
                "local" => Channel::Local,
                "npc" => Channel::Npc,
                "starsystem" => Channel::StarSystem,
                "squadron" => Channel::Squadron,
                "squadleaders" => Channel::SquadLeaders,
                _ => panic!("Unknown message channel: {}", value.channel),
            },
        }
    }
}

static NPC_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$npc_name_decorate:#name=([^;]+);$").unwrap());
static SYSTEM_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$CHAT_([^;]+);$").unwrap());
static SHIP_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$ShipName_([^;]+);$").unwrap());

fn sanitize_name(name: &String) -> (String, Kind) {
    if let Some(caps) = NPC_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().to_string(), Kind::Npc);
    }
    if let Some(caps) = SYSTEM_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().to_string(), Kind::System);
    }
    if let Some(caps) = SHIP_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().to_string().replace("_", ": "), Kind::Ship);
    }
    (name.to_string(), Kind::Chat)
}