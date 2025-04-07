use crate::event::format::prettify_date;
use crate::state::{Channel, ChatMessage, Kind};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SendText {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "To")]
    pub to: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Sent")]
    pub sent: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReceiveText {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "From")]
    pub from: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Message_Localised")]
    pub message_localised: Option<String>,

    #[serde(rename = "Channel")]
    pub channel: String
}

impl Into<ChatMessage> for ReceiveText {
    fn into(self) -> ChatMessage {
        let (text, kind) = sanitize_name(&self.from);

        ChatMessage {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            text: self.message_localised.unwrap_or(self.message),
            from: text,
            kind,
            channel: match self.channel.as_str() {
                "local" => Channel::Local,
                "npc" => Channel::Npc,
                "starsystem" => Channel::StarSystem,
                "squadron" => Channel::Squadron,
                "squadleaders" => Channel::SquadLeaders,
                _ => panic!("Unknown message channel: {}", self.channel)
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