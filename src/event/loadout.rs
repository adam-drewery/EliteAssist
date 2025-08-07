use crate::{event, state};
use crate::fdev_ids::Shipyard;

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
                None => String::new()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

impl Into<state::SuitLoadout> for event::SuitLoadout {
    fn into(self) -> state::SuitLoadout {
        state::SuitLoadout {
            timestamp: self.timestamp,
            suit_id: self.suit_id,
            suit_name: localized_name(self.suit_name_localised.unwrap_or_else(|| self.suit_name)),
            suit_mods: self.suit_mods.into_iter().map(|m| m.into()).collect(),
            loadout_id: self.loadout_id,
            loadout_name: self.loadout_name,
            modules: self.modules.into_iter().map(|m| m.into()).collect(),
        }
    }
}

impl Into<state::SuitModule> for event::SuitLoadoutModule {
    fn into(self) -> state::SuitModule {
        state::SuitModule {
            slot_name: self.slot_name,
            suit_module_id: self.suit_module_id,
            module_name: self.module_name_localised.unwrap_or(self.module_name),
            class: self.class,
            weapon_mods: self.weapon_mods,
        }
    }
}

impl Into<state::ShipLoadout> for event::Loadout {
    fn into(self) -> state::ShipLoadout {

        let ship_type = Shipyard::metadata(&self.ship);
        
        state::ShipLoadout {
            timestamp: self.timestamp,
            ship_type: ship_type.map(|s| s.name.clone()).unwrap_or(self.ship),
            ship_id: self.ship_id,
            ship_name: self.ship_name,
            ship_ident: self.ship_ident,
            hull_value: self.hull_value.unwrap_or_default(),
            modules_value: self.modules_value.unwrap_or_default(),
            hull_health: self.hull_health,
            unladen_mass: self.unladen_mass,
            cargo_capacity: self.cargo_capacity,
            max_jump_range: self.max_jump_range,
            fuel_capacity: state::FuelCapacity {
                main: self.fuel_capacity.main,
                reserve: self.fuel_capacity.reserve,
            },
            modules: self.modules.into_iter().map(|m| m.into()).collect(),
            rebuy: self.rebuy,
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