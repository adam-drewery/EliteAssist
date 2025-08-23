use ed_journals::logs::suit_loadout_event::SuitLoadoutEvent;
use ed_journals::odyssey::{SuitMod, SuitSlot};

#[derive(Default)]
pub struct SuitLoadout {

    pub suit_name: String,
    pub suit_mods: Vec<String>,
    pub loadout_name: String,
    pub modules: Vec<SuitModule>,
}

#[derive(Default)]
pub struct SuitModule {

    pub slot_name: String,
    pub class: u8,
}

fn localized_name(name: String) -> String {
    if !name.starts_with('$') {
        return name;
    }
    name.trim_end_matches("_Name;")
        .trim_start_matches('$')
        .split('_')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().chain(chars.flat_map(|c| c.to_lowercase())).collect(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

impl From<SuitLoadoutEvent> for SuitLoadout {
    fn from(value: SuitLoadoutEvent) -> Self {
        SuitLoadout {
            loadout_name: value.loadout_name,
            suit_name: value.suit_name_localized,
            suit_mods: value.suit_mods.into_iter().map(|x| {
                match x {
                    SuitMod::ReducedToolBatteryConsumption => "Reduced Tool Battery Consumption",
                    SuitMod::ImprovedBatteryCapacity => "Improved Battery Capacity",
                    SuitMod::IncreasedSprintDuration => "Increased Sprint Duration",
                    SuitMod::CombatMovementSpeed => "Combat Movement Speed",
                    SuitMod::ImprovedJumpAssist => "Improved Jump Assist",
                    SuitMod::IncreasedAirReserves => "Increased Air Reserves",
                    SuitMod::NightVision => "Night Vision",
                    SuitMod::EnhancedTracking => "Enhanced Tracking",
                    SuitMod::ExtraBackpackCapacity => "Extra Backpack Capacity",
                    SuitMod::AddedMeleeDamage => "Added Melee Damage",
                    SuitMod::DamageResistance => "Damage Resistance",
                    SuitMod::ExtraAmmoCapacity => "Extra Ammo Capacity",
                    SuitMod::FasterShieldRegen => "Faster Shield Regen",
                    SuitMod::QuieterFootsteps => "Quieter Footsteps"
                }.to_string()
            }).collect(),
            modules: value.modules.into_iter().map(|m| SuitModule {
                slot_name: match m.slot_name {
                    SuitSlot::PrimaryWeapon1 => "Primary Weapon 1".to_string(),
                    SuitSlot::PrimaryWeapon2 => "Primary Weapon 2".to_string(),
                    SuitSlot::SecondaryWeapon => "Secondary Weapon".to_string(),
                },
                class: value.suit_name.class,
            }).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localized_name() {
        assert_eq!(localized_name("$python_PILOT_Name;".to_string()), "Python Pilot");
        assert_eq!(localized_name("$SIDEWINDER_Name;".to_string()), "Sidewinder");
        assert_eq!(localized_name("$BIG_COOL_SHIP_Name;".to_string()), "Big Cool Ship");
        assert_eq!(localized_name("Regular Name".to_string()), "Regular Name");
        assert_eq!(localized_name("$".to_string()), "");
        assert_eq!(localized_name("$SINGLE".to_string()), "Single");
    }
}