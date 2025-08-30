use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Stations {
    pub id: i64,
    pub id64: i64,
    pub name: Box<str>,
    pub url: Box<str>,
    pub stations: Vec<Station>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Station {
    pub id: i64,
    #[serde(rename = "marketId")]
    pub market_id: i64,
    #[serde(rename = "type")]
    pub r#type: Box<str>,
    pub name: Box<str>,
    pub body: Option<Body>,
    #[serde(rename = "distanceToArrival")]
    pub distance_to_arrival: f32,
    pub allegiance: Box<str>,
    pub government: Box<str>,
    pub economy: Box<str>,
    #[serde(rename = "secondEconomy")]
    pub second_economy: Option<Box<str>>,
    #[serde(rename = "haveMarket")]
    pub have_market: bool,
    #[serde(rename = "haveShipyard")]
    pub have_shipyard: bool,
    #[serde(rename = "haveOutfitting")]
    pub have_outfitting: bool,
    #[serde(rename = "otherServices")]
    pub other_services: Vec<Box<str>>,
    #[serde(rename = "controllingFaction")]
    pub controlling_faction: Option<ControllingFaction>,
    #[serde(rename = "updateTime")]
    pub update_time: UpdateTime,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateTime {
    pub information: Box<str>,
    pub market: Option<Box<str>>,
    pub shipyard: Option<Box<str>>,
    pub outfitting: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ControllingFaction {

    // is null when it's actually an engineer controlling the place.
    pub id: Option<u64>,
    pub name: Box<str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Body {
    pub id: i64,
    pub name: Box<str>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
}
