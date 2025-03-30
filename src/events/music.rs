use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Music {

    pub timestamp: String,

    #[serde(rename = "MusicTrack")]
    pub music_track: String,
}