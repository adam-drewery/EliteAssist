use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Stations {
    pub id: i64,
    pub id64: i64,
    pub name: String,
    pub url: String,
    pub stations: Vec<Station>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Station {
    pub id: i64,
    #[serde(rename = "marketId")]
    pub market_id: i64,
    #[serde(rename = "type")]
    pub r#type: String,
    pub name: String,
    pub body: Option<Body>,
    #[serde(rename = "distanceToArrival")]
    pub distance_to_arrival: f32,
    pub allegiance: String,
    pub government: String,
    pub economy: String,
    #[serde(rename = "secondEconomy")]
    pub second_economy: Option<String>,
    #[serde(rename = "haveMarket")]
    pub have_market: bool,
    #[serde(rename = "haveShipyard")]
    pub have_shipyard: bool,
    #[serde(rename = "haveOutfitting")]
    pub have_outfitting: bool,
    #[serde(rename = "otherServices")]
    pub other_services: Vec<String>,
    #[serde(rename = "controllingFaction")]
    pub controlling_faction: ControllingFaction,
    #[serde(rename = "updateTime")]
    pub update_time: UpdateTime,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateTime {
    pub information: String,
    pub market: Option<String>,
    pub shipyard: Option<String>,
    pub outfitting: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ControllingFaction {

    // is null when it's actually an engineer controlling the place.
    pub id: Option<u64>,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Body {
    pub id: i64,
    pub name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}
