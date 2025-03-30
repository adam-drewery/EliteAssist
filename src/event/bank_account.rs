use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
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