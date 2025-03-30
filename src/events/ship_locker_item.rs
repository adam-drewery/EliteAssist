use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ShipLockerItem {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    #[serde(rename = "MissionID")]
    pub mission_id: Option<u64>,

    #[serde(rename = "Count")]
    pub count: u64,
}

impl ShipLockerItem {
    pub fn display_name(&self) -> String {
        self.name_localised.clone().unwrap_or(title_case(&self.name))
    }
}

fn title_case(s: &String) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().chain(c).collect(),
        None => String::new()
    }
}