use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct AppliedToSquardon {

    pub timestamp: String,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,
}