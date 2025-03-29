use serde::Deserialize;
use crate::events::{BankAccount, Combat, Crime, Exploration, Mining, Passengers, Smuggling, Trading};
use crate::events::cqc::Cqc;
use crate::events::crafting::Crafting;
use crate::events::crew::Crew;
use crate::events::exobiology::Exobiology;
use crate::events::material_trader_stats::MaterialTraderStats;
use crate::events::multicrew::Multicrew;
use crate::events::search_and_rescue::SearchAndRescue;
use crate::events::tg_encounters::TgEncounters;

#[derive(Deserialize, Debug)]
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