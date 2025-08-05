use crate::event;
use crate::state;


impl Into<state::SuitLoadout> for event::SuitLoadout {

    fn into(self) -> state::SuitLoadout {
        state::SuitLoadout {
            timestamp: self.timestamp,
            suit_id: self.suit_id,
            suit_name: self.suit_name_localised.unwrap_or(self.suit_name),
            suit_mods: vec![],
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
            ship: self.ship,
            modules: self.modules.into_iter().map(|m| {
                // something's wrong. this struct represents a suit loadout not a ship loadout.
                
            }).collect(),
        }
    }
} 