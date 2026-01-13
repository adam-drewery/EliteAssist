use log::error;
use crate::journal::event;
use crate::lookup;
use crate::lookup::SuitClass;

#[derive(Default)]
pub struct Loadout {
    pub suit_name: &'static str,
    pub class: u8,
    pub suit_mods: Vec<&'static str>,
    pub loadout_name: Box<str>,
    pub modules: Vec<Module>,
}

#[derive(Default)]
pub struct Module {
    pub slot_name: Box<str>,
    pub module_name: Box<str>,
    pub class: u64,
    pub weapon_mods: Vec<&'static str>,
}

impl From<event::SuitLoadoutModule> for Module {
    fn from(value: event::SuitLoadoutModule) -> Self {
        Module {
            slot_name: value.slot_name,
            module_name: value.module_name_localised.unwrap_or(value.module_name),
            class: value.class,
            weapon_mods: value.weapon_mods.into_iter().map(|m| {
                *lookup::SUIT_MODULE_NAMES.get(m.as_ref())
                    .unwrap_or_else(|| {
                        error!("Error: Missing suit module name: {}", m);
                        &"Unknown"
                    })
            }).collect(),
        }
    }
}

impl From<event::SuitLoadout> for Loadout {
    fn from(value: event::SuitLoadout) -> Self {

        let suit_class = lookup::SUIT_CLASS_NAMES.get(&value.suit_name)
            .unwrap_or_else(|| {
                error!("Error: Missing suit module name: {}", value.suit_name);
                &SuitClass { name: "Unknown", rank: 0 }
            });

        Loadout {
            suit_name: suit_class.name,
            class: suit_class.rank,
            suit_mods: value.suit_mods.into_iter().map(|m| {
                *lookup::SUIT_MODULE_NAMES.get(m.as_ref())
                    .unwrap_or_else(|| {
                        error!("Error: Missing suit module name: {}", m);
                        &"Unknown"
                    })
            }).collect(),
            loadout_name: value.loadout_name,
            modules: value.modules.into_iter().map(|m| m.into()).collect(),
        }
    }
}