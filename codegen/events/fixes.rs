use std::collections::HashMap;
use std::sync::OnceLock;

macro_rules! struct_name_map {
    ( $( [$($key:expr),*] => $value:expr ),* ) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert(vec![$($key),*], $value);
            )*
            map
        }
    };
}

static STRUCT_NAME_MERGES: OnceLock<HashMap<Vec<&'static str>, &'static str>> = OnceLock::new();

pub fn struct_name_merges() -> &'static HashMap<Vec<&'static str>, &'static str> {
    STRUCT_NAME_MERGES.get_or_init(|| {
        struct_name_map! {
            ["JoinACrew", "QuitACrew"] => "Crew",
            ["FighterDestroyed", "HeatDamage"] => "Damage",
            ["BuySuit", "SellSuit"] => "Suit",
            ["PayBounties", "PayFines"] => "Payment",
            ["Commodities", "CommodityReward", "Encoded", "Ingredients", "Manufactured", "Materials", "Raw", "Resources"] => "Material",
            ["Backpack", "ShipLocker"] => "Inventory",
            ["Active", "Complete", "Failed"] => "MissionStatus"
        }
    })
}