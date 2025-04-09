use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Statistics {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

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

#[derive(Clone, Debug, Deserialize)]
pub struct Crafting {

    #[serde(rename = "Count_Of_Used_Engineers")]
    pub count_of_used_engineers: u64,

    #[serde(rename = "Recipes_Generated")]
    pub recipes_generated: u64,

    #[serde(rename = "Recipes_Generated_Rank_1")]
    pub recipes_generated_rank_1: u64,

    #[serde(rename = "Recipes_Generated_Rank_2")]
    pub recipes_generated_rank_2: u64,

    #[serde(rename = "Recipes_Generated_Rank_3")]
    pub recipes_generated_rank_3: u64,

    #[serde(rename = "Recipes_Generated_Rank_4")]
    pub recipes_generated_rank_4: u64,

    #[serde(rename = "Recipes_Generated_Rank_5")]
    pub recipes_generated_rank_5: u64,

    #[serde(rename = "Suit_Mods_Applied")]
    pub suit_mods_applied: u64,

    #[serde(rename = "Weapon_Mods_Applied")]
    pub weapon_mods_applied: u64,

    #[serde(rename = "Suits_Upgraded")]
    pub suits_upgraded: u64,

    #[serde(rename = "Weapons_Upgraded")]
    pub weapons_upgraded: u64,

    #[serde(rename = "Suits_Upgraded_Full")]
    pub suits_upgraded_full: u64,

    #[serde(rename = "Weapons_Upgraded_Full")]
    pub weapons_upgraded_full: u64,

    #[serde(rename = "Suit_Mods_Applied_Full")]
    pub suit_mods_applied_full: u64,

    #[serde(rename = "Weapon_Mods_Applied_Full")]
    pub weapon_mods_applied_full: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Exobiology {

    #[serde(rename = "Organic_Genus_Encountered")]
    pub organic_genus_encountered: u64,

    #[serde(rename = "Organic_Species_Encountered")]
    pub organic_species_encountered: u64,

    #[serde(rename = "Organic_Variant_Encountered")]
    pub organic_variant_encountered: u64,

    #[serde(rename = "Organic_Data_Profits")]
    pub organic_data_profits: u64,

    #[serde(rename = "Organic_Data")]
    pub organic_data: u64,

    #[serde(rename = "First_Logged_Profits")]
    pub first_logged_profits: u64,

    #[serde(rename = "First_Logged")]
    pub first_logged: u64,

    #[serde(rename = "Organic_Systems")]
    pub organic_systems: u64,

    #[serde(rename = "Organic_Planets")]
    pub organic_planets: u64,

    #[serde(rename = "Organic_Genus")]
    pub organic_genus: u64,

    #[serde(rename = "Organic_Species")]
    pub organic_species: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BankAccount {

    #[serde(rename = "Current_Wealth")]
    pub current_wealth: u64,

    #[serde(rename = "Spent_On_Ships")]
    pub spent_on_ships: u64,

    #[serde(rename = "Spent_On_Outfitting")]
    pub spent_on_outfitting: u64,

    #[serde(rename = "Spent_On_Repairs")]
    pub spent_on_repairs: u64,

    #[serde(rename = "Spent_On_Fuel")]
    pub spent_on_fuel: u64,

    #[serde(rename = "Spent_On_Ammo_Consumables")]
    pub spent_on_ammo_consumables: u64,

    #[serde(rename = "Insurance_Claims")]
    pub insurance_claims: u64,

    #[serde(rename = "Spent_On_Insurance")]
    pub spent_on_insurance: u64,

    #[serde(rename = "Owned_Ship_Count")]
    pub owned_ship_count: u64,

    #[serde(rename = "Spent_On_Suits")]
    pub spent_on_suits: u64,

    #[serde(rename = "Spent_On_Weapons")]
    pub spent_on_weapons: u64,

    #[serde(rename = "Spent_On_Suit_Consumables")]
    pub spent_on_suit_consumables: u64,

    #[serde(rename = "Suits_Owned")]
    pub suits_owned: u64,

    #[serde(rename = "Weapons_Owned")]
    pub weapons_owned: u64,

    #[serde(rename = "Spent_On_Premium_Stock")]
    pub spent_on_premium_stock: u64,

    #[serde(rename = "Premium_Stock_Bought")]
    pub premium_stock_bought: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Cqc {

    #[serde(rename = "CQC_Credits_Earned")]
    pub cqc_credits_earned: u64,

    #[serde(rename = "CQC_Time_Played")]
    pub cqc_time_played: u64,

    #[serde(rename = "CQC_KD")]
    pub cqc_kd: f64,

    #[serde(rename = "CQC_Kills")]
    pub cqc_kills: u64,

    #[serde(rename = "CQC_WL")]
    pub cqc_wl: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Combat {

    #[serde(rename = "Bounties_Claimed")]
    pub bounties_claimed: u64,

    #[serde(rename = "Bounty_Hunting_Profit")]
    pub bounty_hunting_profit: u64,

    #[serde(rename = "Combat_Bonds")]
    pub combat_bonds: u64,

    #[serde(rename = "Combat_Bond_Profits")]
    pub combat_bond_profits: u64,

    #[serde(rename = "Assassinations")]
    pub assassinations: u64,

    #[serde(rename = "Assassination_Profits")]
    pub assassination_profits: u64,

    #[serde(rename = "Highest_Single_Reward")]
    pub highest_single_reward: u64,

    #[serde(rename = "Skimmers_Killed")]
    pub skimmers_killed: u64,

    #[serde(rename = "OnFoot_Combat_Bonds")]
    pub on_foot_combat_bonds: u64,

    #[serde(rename = "OnFoot_Combat_Bonds_Profits")]
    pub on_foot_combat_bonds_profits: u64,

    #[serde(rename = "OnFoot_Vehicles_Destroyed")]
    pub on_foot_vehicles_destroyed: u64,

    #[serde(rename = "OnFoot_Ships_Destroyed")]
    pub on_foot_ships_destroyed: u64,

    #[serde(rename = "Dropships_Taken")]
    pub dropships_taken: u64,

    #[serde(rename = "Dropships_Booked")]
    pub dropships_booked: u64,

    #[serde(rename = "Dropships_Cancelled")]
    pub dropships_cancelled: u64,

    #[serde(rename = "ConflictZone_High")]
    pub conflict_zone_high: u64,

    #[serde(rename = "ConflictZone_Medium")]
    pub conflict_zone_medium: u64,

    #[serde(rename = "ConflictZone_Low")]
    pub conflict_zone_low: u64,

    #[serde(rename = "ConflictZone_Total")]
    pub conflict_zone_total: u64,

    #[serde(rename = "ConflictZone_High_Wins")]
    pub conflict_zone_high_wins: u64,

    #[serde(rename = "ConflictZone_Medium_Wins")]
    pub conflict_zone_medium_wins: u64,

    #[serde(rename = "ConflictZone_Low_Wins")]
    pub conflict_zone_low_wins: u64,

    #[serde(rename = "ConflictZone_Total_Wins")]
    pub conflict_zone_total_wins: u64,

    #[serde(rename = "Settlement_Defended")]
    pub settlement_defended: u64,

    #[serde(rename = "Settlement_Conquered")]
    pub settlement_conquered: u64,

    #[serde(rename = "OnFoot_Skimmers_Killed")]
    pub on_foot_skimmers_killed: u64,

    #[serde(rename = "OnFoot_Scavs_Killed")]
    pub on_foot_scavs_killed: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Crime {

    #[serde(rename = "Notoriety")]
    pub notoriety: u64,

    #[serde(rename = "Fines")]
    pub fines: u64,

    #[serde(rename = "Total_Fines")]
    pub total_fines: u64,

    #[serde(rename = "Bounties_Received")]
    pub bounties_received: u64,

    #[serde(rename = "Total_Bounties")]
    pub total_bounties: u64,

    #[serde(rename = "Highest_Bounty")]
    pub highest_bounty: u64,

    #[serde(rename = "Malware_Uploaded")]
    pub malware_uploaded: u64,

    #[serde(rename = "Settlements_State_Shutdown")]
    pub settlements_state_shutdown: u64,

    #[serde(rename = "Production_Sabotage")]
    pub production_sabotage: u64,

    #[serde(rename = "Production_Theft")]
    pub production_theft: u64,

    #[serde(rename = "Total_Murders")]
    pub total_murders: u64,

    #[serde(rename = "Citizens_Murdered")]
    pub citizens_murdered: u64,

    #[serde(rename = "Omnipol_Murdered")]
    pub omnipol_murdered: u64,

    #[serde(rename = "Guards_Murdered")]
    pub guards_murdered: u64,

    #[serde(rename = "Data_Stolen")]
    pub data_stolen: u64,

    #[serde(rename = "Goods_Stolen")]
    pub goods_stolen: u64,

    #[serde(rename = "Sample_Stolen")]
    pub sample_stolen: u64,

    #[serde(rename = "Total_Stolen")]
    pub total_stolen: u64,

    #[serde(rename = "Turrets_Destroyed")]
    pub turrets_destroyed: u64,

    #[serde(rename = "Turrets_Overloaded")]
    pub turrets_overloaded: u64,

    #[serde(rename = "Turrets_Total")]
    pub turrets_total: u64,

    #[serde(rename = "Value_Stolen_StateChange")]
    pub value_stolen_state_change: u64,

    #[serde(rename = "Profiles_Cloned")]
    pub profiles_cloned: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Smuggling {

    #[serde(rename = "Black_Markets_Traded_With")]
    pub black_markets_traded_with: u64,

    #[serde(rename = "Black_Markets_Profits")]
    pub black_markets_profits: u64,

    #[serde(rename = "Resources_Smuggled")]
    pub resources_smuggled: u64,

    #[serde(rename = "Average_Profit")]
    pub average_profit: f64,

    #[serde(rename = "Highest_Single_Transaction")]
    pub highest_single_transaction: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Trading {

    #[serde(rename = "Markets_Traded_With")]
    pub markets_traded_with: u64,

    #[serde(rename = "Market_Profits")]
    pub market_profits: u64,

    #[serde(rename = "Resources_Traded")]
    pub resources_traded: u64,

    #[serde(rename = "Average_Profit")]
    pub average_profit: f64,

    #[serde(rename = "Highest_Single_Transaction")]
    pub highest_single_transaction: u64,

    #[serde(rename = "Data_Sold")]
    pub data_sold: u64,

    #[serde(rename = "Goods_Sold")]
    pub goods_sold: u64,

    #[serde(rename = "Assets_Sold")]
    pub assets_sold: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mining {

    #[serde(rename = "Mining_Profits")]
    pub mining_profits: u64,

    #[serde(rename = "Quantity_Mined")]
    pub quantity_mined: u64,

    #[serde(rename = "Materials_Collected")]
    pub materials_collected: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Exploration {

    #[serde(rename = "Systems_Visited")]
    pub systems_visited: u64,

    #[serde(rename = "Exploration_Profits")]
    pub exploration_profits: u64,

    #[serde(rename = "Planets_Scanned_To_Level_2")]
    pub planets_scanned_to_level_2: u64,

    #[serde(rename = "Planets_Scanned_To_Level_3")]
    pub planets_scanned_to_level_3: u64,

    #[serde(rename = "Efficient_Scans")]
    pub efficient_scans: u64,

    #[serde(rename = "Highest_Payout")]
    pub highest_payout: u64,

    #[serde(rename = "Total_Hyperspace_Distance")]
    pub total_hyperspace_distance: u64,

    #[serde(rename = "Total_Hyperspace_Jumps")]
    pub total_hyperspace_jumps: u64,

    #[serde(rename = "Greatest_Distance_From_Start")]
    pub greatest_distance_from_start: f64,

    #[serde(rename = "Time_Played")]
    pub time_played: u64,

    #[serde(rename = "OnFoot_Distance_Travelled")]
    pub on_foot_distance_travelled: u64,

    #[serde(rename = "Shuttle_Journeys")]
    pub shuttle_journeys: u64,

    #[serde(rename = "Shuttle_Distance_Travelled")]
    pub shuttle_distance_travelled: f64,

    #[serde(rename = "Spent_On_Shuttles")]
    pub spent_on_shuttles: u64,

    #[serde(rename = "First_Footfalls")]
    pub first_footfalls: u64,

    #[serde(rename = "Planet_Footfalls")]
    pub planet_footfalls: u64,

    #[serde(rename = "Settlements_Visited")]
    pub settlements_visited: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Passengers {

    #[serde(rename = "Passengers_Missions_Accepted")]
    pub passengers_missions_accepted: u64,

    #[serde(rename = "Passengers_Missions_Disgruntled")]
    pub passengers_missions_disgruntled: u64,

    #[serde(rename = "Passengers_Missions_Bulk")]
    pub passengers_missions_bulk: u64,

    #[serde(rename = "Passengers_Missions_VIP")]
    pub passengers_missions_vip: u64,

    #[serde(rename = "Passengers_Missions_Delivered")]
    pub passengers_missions_delivered: u64,

    #[serde(rename = "Passengers_Missions_Ejected")]
    pub passengers_missions_ejected: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SearchAndRescue {

    #[serde(rename = "SearchRescue_Traded")]
    pub search_rescue_traded: u64,

    #[serde(rename = "SearchRescue_Profit")]
    pub search_rescue_profit: u64,

    #[serde(rename = "SearchRescue_Count")]
    pub search_rescue_count: u64,

    #[serde(rename = "Salvage_Legal_POI")]
    pub salvage_legal_poi: u64,

    #[serde(rename = "Salvage_Legal_Settlements")]
    pub salvage_legal_settlements: u64,

    #[serde(rename = "Salvage_Illegal_POI")]
    pub salvage_illegal_poi: u64,

    #[serde(rename = "Salvage_Illegal_Settlements")]
    pub salvage_illegal_settlements: u64,

    #[serde(rename = "Maglocks_Opened")]
    pub maglocks_opened: u64,

    #[serde(rename = "Panels_Opened")]
    pub panels_opened: u64,

    #[serde(rename = "Settlements_State_FireOut")]
    pub settlements_state_fire_out: u64,

    #[serde(rename = "Settlements_State_Reboot")]
    pub settlements_state_reboot: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TgEncounters {

    #[serde(rename = "TG_ENCOUNTER_KILLED")]
    pub tg_encounter_killed: Option<u64>,

    #[serde(rename = "TG_ENCOUNTER_TOTAL")]
    pub tg_encounter_total: Option<u64>,

    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SYSTEM")]
    pub tg_encounter_total_last_system: Option<String>,

    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_TIMESTAMP")]
    pub tg_encounter_total_last_timestamp: Option<String>,

    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SHIP")]
    pub tg_encounter_total_last_ship: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Crew {

    #[serde(rename = "NpcCrew_TotalWages")]
    pub npc_crew_total_wages: u64,

    #[serde(rename = "NpcCrew_Hired")]
    pub npc_crew_hired: u64,

    #[serde(rename = "NpcCrew_Fired")]
    pub npc_crew_fired: u64,

    #[serde(rename = "NpcCrew_Died")]
    pub npc_crew_died: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Multicrew {

    #[serde(rename = "Multicrew_Time_Total")]
    pub multicrew_time_total: u64,

    #[serde(rename = "Multicrew_Gunner_Time_Total")]
    pub multicrew_gunner_time_total: u64,

    #[serde(rename = "Multicrew_Fighter_Time_Total")]
    pub multicrew_fighter_time_total: u64,

    #[serde(rename = "Multicrew_Credits_Total")]
    pub multicrew_credits_total: u64,

    #[serde(rename = "Multicrew_Fines_Total")]
    pub multicrew_fines_total: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MaterialTraderStats {

    #[serde(rename = "Trades_Completed")]
    pub trades_completed: u64,

    #[serde(rename = "Materials_Traded")]
    pub materials_traded: u64,

    #[serde(rename = "Encoded_Materials_Traded")]
    pub encoded_materials_traded: u64,

    #[serde(rename = "Raw_Materials_Traded")]
    pub raw_materials_traded: u64,

    #[serde(rename = "Grade_1_Materials_Traded")]
    pub grade_1_materials_traded: u64,

    #[serde(rename = "Grade_2_Materials_Traded")]
    pub grade_2_materials_traded: u64,

    #[serde(rename = "Grade_3_Materials_Traded")]
    pub grade_3_materials_traded: u64,

    #[serde(rename = "Grade_4_Materials_Traded")]
    pub grade_4_materials_traded: u64,

    #[serde(rename = "Grade_5_Materials_Traded")]
    pub grade_5_materials_traded: u64,

    #[serde(rename = "Assets_Traded_In")]
    pub assets_traded_in: u64,

    #[serde(rename = "Assets_Traded_Out")]
    pub assets_traded_out: u64,
}