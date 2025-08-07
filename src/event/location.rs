use crate::{event, state};

impl Into<state::CurrentLocation> for event::Location {
    fn into(self) -> state::CurrentLocation {
        state::CurrentLocation {
            dist_from_star_ls: self.dist_from_star_ls,
            docked: self.docked,
            station_name: self.station_name,
            station_type: self.station_type,
            station_faction: self.station_faction.map(|sf| state::SystemFaction {
                name: sf.name,
                faction_state: sf.faction_state,
            }),
            station_government: self.station_government_localised,
            station_services: self.station_services,
            station_economy: self.station_economy_localised,
            station_economies: self
                .station_economies
                .unwrap_or_default()
                .into_iter()
                .map(|economy| state::StationEconomy {
                    name: economy.name_localised.unwrap_or_default(),
                    proportion: economy.proportion,
                })
                .collect(),
            taxi: self.taxi,
            multicrew: self.multicrew,
            star_system: self.star_system,
            system_address: self.system_address,
            star_pos: self.star_pos,
            system_allegiance: self.system_allegiance,
            system_economy: self.system_economy_localised.unwrap_or(self.system_economy),
            system_second_economy: self
                .system_second_economy_localised
                .unwrap_or(self.system_second_economy),
            system_government: self
                .system_government_localised
                .unwrap_or(self.system_government),
            system_security: self
                .system_security_localised
                .unwrap_or(self.system_security),
            population: self.population,
            body: self.body,
            body_id: self.body_id,
            body_type: self.body_type,
            controlling_power: self.controlling_power,
            powers: self.powers,
            powerplay_state: self.powerplay_state,
            powerplay_state_conflict_progress: None, // todo
            powerplay_state_control_progress: self.powerplay_state_control_progress,
            powerplay_state_reinforcement: self.powerplay_state_reinforcement,
            powerplay_state_undermining: self.powerplay_state_undermining,
            factions: self
                .factions
                .unwrap_or_default()
                .into_iter()
                .map(|faction| state::Faction {
                    name: faction.name,
                    faction_state: faction.faction_state,
                    government: faction.government,
                    influence: faction.influence,
                    allegiance: faction.allegiance,
                    happiness: faction.happiness_localised.unwrap_or_default(),
                    my_reputation: faction.my_reputation,
                    recovering_states: faction
                        .recovering_states
                        .unwrap_or_default()
                        .into_iter()
                        .map(|state| state::FactionState {
                                    state: state.state,
                                    trend: state.trend
                        })
                        .collect(),
                    active_states: faction
                        .active_states
                        .unwrap_or_default()
                        .into_iter()
                        .map(|state| state::FactionState {
                                    state: state.state,
                                    trend: 0
                        })
                        .collect(),
                })
                .collect(),
            system_faction: self.system_faction.map(|sf| state::SystemFaction {
                name: sf.name,
                faction_state: sf.faction_state,
            }),
        }
    }
}
