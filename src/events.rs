use std::fmt;
use std::fmt::Formatter;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileHeader {
    pub timestamp: String,
    pub part: u8,
    pub language: String,
    #[serde(rename = "gameversion")]
    pub game_version: String,
    pub build: String
}

impl fmt::Display for FileHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Language: {}, Game Version: {} ({})", self.language, self.game_version, self.build)
    }
}

#[derive(Debug, Deserialize)]
pub struct Commander {
    pub timestamp: String,
    #[serde(rename = "FID")]
    pub fid: String,
    #[serde(rename = "Name")]
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct Materials {
    pub timestamp: String,
    pub materials: Vec<Material>
}

#[derive(Debug, Deserialize)]
pub struct Material {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Count")]
    pub count: u8
}

#[derive(Debug, Deserialize)]
pub struct Rank {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct Progress {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct Reputation {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct EngineerProgress {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct SquadronStartup {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct LoadGame {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct Statistics {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct ReceiveText {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct Powerplay {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct Music {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct SuitLoadout {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct Backpack {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct ShipLocker {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct Missions {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
pub struct Shutdown {
    pub timestamp: String
}

#[derive(Debug, Deserialize)]
#[serde(tag = "event")]
pub enum EliteEvent {

    #[serde(rename = "Fileheader")]
    FileHeader(FileHeader),
    Commander(Commander),
    Materials(Materials),
    Rank(Rank),
    Progress(Progress),
    Reputation(Reputation),
    EngineerProgress(EngineerProgress),
    SquadronStartup(SquadronStartup),
    LoadGame(LoadGame),
    Statistics(Statistics),
    ReceiveText(ReceiveText),
    Location(Location),
    Powerplay(Powerplay),
    Music(Music),
    SuitLoadout(SuitLoadout),
    Backpack(Backpack),
    ShipLocker(ShipLocker),
    Missions(Missions),
    Shutdown(Shutdown)
}