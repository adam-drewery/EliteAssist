use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct JetConeBoost {

    pub timestamp: String,

    #[serde(rename = "BoostValue")]
    pub boost_value: f64,
}