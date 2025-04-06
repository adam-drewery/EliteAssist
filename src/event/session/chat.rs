use crate::event::format::prettify_date;
use crate::state::{ChatMessage, MessageKind};
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
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

#[derive(Deserialize, Debug, Default, Clone)]
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
            channel: self.channel,
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