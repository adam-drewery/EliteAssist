use crate::{event, state};

impl Into<state::SuitLoadout> for event::SuitLoadout {

    fn into(self) -> state::SuitLoadout {
        state::SuitLoadout {
            timestamp: self.timestamp,
            suit_id: self.suit_id,
            suit_name: self.suit_name_localised.unwrap_or(self.suit_name),
            suit_mods: self.modules.into_iter().map(|m| m.module_name).collect(),
            loadout_id: self.loadout_id,
            loadout_name: self.loadout_name,
            modules: vec![],
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
        state::ShipLoadout {
            timestamp: self.timestamp,
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
            ship: self.ship,
            modules: self.modules.into_iter().map(|m| {
                m.into()
            }).collect(),
            rebuy: self.rebuy,
        }
    }
} 