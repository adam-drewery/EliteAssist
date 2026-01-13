use log::warn;
use crate::journal::event;
use crate::journal::format::prettify_date;
use once_cell::sync::Lazy;
use regex::Regex;

pub struct Message {
    pub timestamp: i64,
    pub time_display: Box<str>,
    pub from: Box<str>,
    pub text: Box<str>,
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
    Player,
    StarSystem,
    Squadron,
    SquadLeaders,
    Wing,
    Unknown
}

impl From<event::ReceiveText> for Message {
    fn from(value: event::ReceiveText) -> Self {
        let (text, _kind) = sanitize_name(value.from.as_ref());
        Message {
            timestamp: value.timestamp.timestamp(),
            time_display: prettify_date(&value.timestamp),
            text: value.message_localised.unwrap_or(value.message),
            from: text,
            channel: match value.channel.as_ref() {
                "local" => Channel::Local,
                "npc" => Channel::Npc,
                "player" => Channel::Player,
                "starsystem" => Channel::StarSystem,
                "squadron" => Channel::Squadron,
                "squadleaders" => Channel::SquadLeaders,
                "wing" => Channel::Wing,
                _ => {
                    warn!("Unknown message channel: {}", value.channel);
                    Channel::Unknown
                },
            },
        }
    }
}

static NPC_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$npc_name_decorate:#name=([^;]+);$").unwrap());
static SYSTEM_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$CHAT_([^;]+);$").unwrap());
static SHIP_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$ShipName_([^;]+);$").unwrap());

fn sanitize_name(name: &str) -> (Box<str>, Kind) {
    if let Some(caps) = NPC_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().into(), Kind::Npc);
    }
    if let Some(caps) = SYSTEM_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().into(), Kind::System);
    }
    if let Some(caps) = SHIP_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().replace("_", ": ").into(), Kind::Ship);
    }
    (name.into(), Kind::Chat)
}