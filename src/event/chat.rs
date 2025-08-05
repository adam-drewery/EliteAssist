use once_cell::sync::Lazy;
use regex::Regex;
use crate::{event, state};
use crate::event::format::prettify_date;

impl Into<state::ChatMessage> for event::ReceiveText {
    fn into(self) -> state::ChatMessage {
        let (text, kind) = sanitize_name(&self.from);

        state::ChatMessage {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            text: self.message_localised.unwrap_or(self.message),
            from: text,
            kind,
            channel: match self.channel.as_str() {
                "local" => state::Channel::Local,
                "npc" => state::Channel::Npc,
                "starsystem" => state::Channel::StarSystem,
                "squadron" => state::Channel::Squadron,
                "squadleaders" => state::Channel::SquadLeaders,
                _ => panic!("Unknown message channel: {}", self.channel)
            },
        }
    }
}

static NPC_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$npc_name_decorate:#name=([^;]+);$").unwrap());
static SYSTEM_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$CHAT_([^;]+);$").unwrap());
static SHIP_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\$ShipName_([^;]+);$").unwrap());

fn sanitize_name(name: &String) -> (String, state::Kind) {
    if let Some(caps) = NPC_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().to_string(), state::Kind::Npc);
    }

    if let Some(caps) = SYSTEM_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().to_string(), state::Kind::System);
    }

    if let Some(caps) = SHIP_NAME.captures(name) {
        return (caps.get(1).unwrap().as_str().to_string().replace("_", ": "), state::Kind::Ship);
    }

    (name.to_string(), state::Kind::Chat)
}