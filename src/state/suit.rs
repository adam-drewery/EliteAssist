use crate::journal::event;

#[derive(Default)]
pub struct SuitLoadout {

    pub suit_name: Box<str>,
    pub suit_mods: Vec<Box<str>>,
    pub loadout_name: Box<str>,
    pub modules: Vec<SuitModule>,
}

#[derive(Default)]
pub struct SuitModule {

    pub slot_name: Box<str>,
    pub module_name: Box<str>,
    pub class: u64,
    pub weapon_mods: Vec<Box<str>>,
}

fn localized_name(name: Box<str>) -> Box<str> {
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
        .into()
}

impl From<event::SuitLoadoutModule> for SuitModule {
    fn from(value: event::SuitLoadoutModule) -> Self {
        SuitModule {
            slot_name: value.slot_name,
            module_name: value.module_name_localised.unwrap_or(value.module_name),
            class: value.class,
            weapon_mods: value.weapon_mods,
        }
    }
}

impl From<event::SuitLoadout> for SuitLoadout {
    fn from(value: event::SuitLoadout) -> Self {
        SuitLoadout {
            suit_name: localized_name(value.suit_name_localised.unwrap_or_else(|| value.suit_name)),
            suit_mods: value.suit_mods.into_iter().map(|m| m.into()).collect(),
            loadout_name: value.loadout_name,
            modules: value.modules.into_iter().map(|m| m.into()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localized_name() {
        assert_eq!(localized_name("$python_PILOT_Name;".into()), "Python Pilot".into());
        assert_eq!(localized_name("$SIDEWINDER_Name;".into()), "Sidewinder".into());
        assert_eq!(localized_name("$BIG_COOL_SHIP_Name;".into()), "Big Cool Ship".into());
        assert_eq!(localized_name("Regular Name".into()), "Regular Name".into());
        assert_eq!(localized_name("$".into()), "".into());
        assert_eq!(localized_name("$SINGLE".into()), "Single".into());
    }
}