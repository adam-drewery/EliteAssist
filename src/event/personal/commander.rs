use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::event::JournalEvent;
use crate::state::State;

#[derive(Deserialize, Debug, Clone)]
#[derive(Default)]
pub struct Commander {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "FID")]
    pub fid: String,

    #[serde(rename = "Name")]
    pub name: String
}

impl JournalEvent for Commander {
    fn handle(self, state: &mut State) {
        state.commander_name = "CMDR ".to_owned() + &self.name.to_uppercase();
    }
}