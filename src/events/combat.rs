use serde::Deserialize;

#[derive(Debug, Deserialize)]
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