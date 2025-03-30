use serde::Deserialize;
use crate::event::{BankAccount, Combat, Crime, Exploration, Mining, Passengers, Smuggling, Trading};
use crate::event::cqc::Cqc;
use crate::event::crafting::Crafting;
use crate::event::crew::Crew;
use crate::event::exobiology::Exobiology;
use crate::event::material_trader_stats::MaterialTraderStats;
use crate::event::multicrew::Multicrew;
use crate::event::search_and_rescue::SearchAndRescue;
use crate::event::tg_encounters::TgEncounters;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Statistics {

    pub timestamp: String,

    #[serde(rename = "Bank_Account")]
    pub bank_account: BankAccount,

    #[serde(rename = "Combat")]
    pub combat: Combat,

    #[serde(rename = "Crime")]
    pub crime: Crime,

    #[serde(rename = "Smuggling")]
    pub smuggling: Smuggling,

    #[serde(rename = "Trading")]
    pub trading: Trading,

    #[serde(rename = "Mining")]
    pub mining: Mining,

    #[serde(rename = "Exploration")]
    pub exploration: Exploration,

    #[serde(rename = "Passengers")]
    pub passengers: Passengers,

    #[serde(rename = "Search_And_Rescue")]
    pub search_and_rescue: SearchAndRescue,

    #[serde(rename = "TG_ENCOUNTERS")]
    pub tg_encounters: TgEncounters,

    #[serde(rename = "Crafting")]
    pub crafting: Crafting,

    #[serde(rename = "Crew")]
    pub crew: Crew,

    #[serde(rename = "Multicrew")]
    pub multicrew: Multicrew,

    #[serde(rename = "Material_Trader_Stats")]
    pub material_trader_stats: MaterialTraderStats,

    #[serde(rename = "CQC")]
    pub cqc: Cqc,

    #[serde(rename = "Exobiology")]
    pub exobiology: Exobiology
}