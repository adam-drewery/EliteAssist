use std::fmt;
use std::fmt::Formatter;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub enum EliteEvent {
    Fileheader {
        timestamp: String,
        part: u8,
        language: String,
        #[serde(rename = "gameversion")]
        game_version: String,
        build: String
    },
    Commander {
        timestamp: String,
        #[serde(rename = "FID")]
        fid: String,
        #[serde(rename = "Name")]
        name: String
    },
    Materials {
        timestamp: String,
    },
    Rank {
        timestamp: String,
    },
    Progress {
        timestamp: String,
    },
    Reputation {
        timestamp: String,
    },
    EngineerProgress {
        timestamp: String,
    },
    SquadronStartup {
        timestamp: String,
    },
    LoadGame {
        timestamp: String,
    },
    Statistics {
        timestamp: String,
    },
    ReceiveText {
        timestamp: String,
    },
    Location {
        timestamp: String,
    },
    Powerplay {
        timestamp: String,
    },
    Music {
        timestamp: String,
    },
    SuitLoadout {
        timestamp: String,
    },
    Backpack {
        timestamp: String,
    },
    ShipLocker {
        timestamp: String,
    },
    Missions {
        timestamp: String,
    },
    Shutdown {
        timestamp: String,
    }
}

impl fmt::Display for EliteEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Event: {:?}", self)
    }
}