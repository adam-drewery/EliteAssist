use crate::event;
use crate::event::format::prettify_date;
use crate::state;

impl Into<state::GameActivity> for event::Embark {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Embarked".to_owned(),
            noun: join_location_parts(&self.star_system, &self.body, &self.station_name),
        }
    }
}

impl Into<state::GameActivity> for event::Disembark {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Disembarked".to_owned(),
            noun: join_location_parts(&self.star_system, &self.body, &self.station_name),
        }
    }
}

fn join_location_parts(system: &String, body: &String, station: &Option<String>) -> String {
    let mut parts = Vec::new();

    if !system.is_empty() {
        parts.push(system.as_str());
    }
    if !body.is_empty() {
        parts.push(body.as_str());
    }
    if let Some(station) = station {
        if !station.is_empty() && !Some(station.to_string()).eq(&Some(body.to_string())) {
            parts.push(station.as_str());
        }
    }
    parts.join(" | ")
}

impl Into<state::CurrentLocation> for event::FSDJump {
    fn into(self) -> state::CurrentLocation {
        state::CurrentLocation {
            dist_from_star_ls: None,
            docked: false,
            station_name: None,
            station_type: None,
            station_faction: None,
            station_government: None,
            station_services: None,
            station_economy: None,
            station_economies: vec![],
            taxi: self.taxi,
            multicrew: self.multicrew,
            star_system: self.star_system,
            system_address: self.system_address,
            star_pos: self.star_pos,
            system_allegiance: self.system_allegiance,
            system_economy: self.system_economy_localised.unwrap_or(self.system_economy),
            system_second_economy: self.system_second_economy_localised.unwrap_or(self.system_second_economy),
            system_government: self.system_government_localised.unwrap_or(self.system_government),
            system_security: self.system_security_localised.unwrap_or(self.system_security),
            population: self.population,
            body: self.body,
            body_id: self.body_id,
            body_type: self.body_type,
            powers: self.powers.clone(),
            controlling_power: self.powers.and_then(|p| p.first().cloned()),
            powerplay_state: self.powerplay_state,
            powerplay_state_conflict_progress: None, // todo
            powerplay_state_control_progress: self.powerplay_state_control_progress,
            powerplay_state_reinforcement: self.powerplay_state_reinforcement,
            powerplay_state_undermining: self.powerplay_state_undermining,
            factions: self.factions.unwrap_or_default().into_iter().map(|f| state::Faction {
                    name: f.name,
                    faction_state: f.faction_state,
                    government: f.government,
                    influence: f.influence,
                    allegiance: f.allegiance,
                    happiness: f.happiness,
                    my_reputation: f.my_reputation,
                    recovering_states: f.recovering_states.unwrap_or_default().into_iter().map(|s| state::FactionState {
                            state: s.state,
                            trend: s.trend
                    }).collect(),
                    active_states: f.active_states.unwrap_or_default().into_iter().map(|s| state::FactionState {
                            state: s.state,
                            trend: 0
                    }).collect()
            }).collect(),
            system_faction: self.system_faction.map(|f| state::SystemFaction {
                name: f.name,
                faction_state: f.faction_state,
            }),
        }
    }
}

impl Into<state::GameActivity> for event::StartJump {
    fn into(self) -> state::GameActivity {

        match self.jump_type.as_str() {
            "Supercruise" => state::GameActivity {
                time: self.timestamp,
                time_display: prettify_date(&self.timestamp),
                verb: "".into(),
                noun: "Entered supercruise".into()
            },
            "Hyperspace" => state::GameActivity {
                time: self.timestamp,
                time_display: prettify_date(&self.timestamp),
                verb: "Jumped to".into(),
                noun: format!["{} ({})", self.star_system.unwrap(), self.star_class.unwrap()]
            },
            _ => panic!("Unknown jump type")
        }
    }
}

impl event::NavRoute {
    pub fn into(self) -> Vec<state::NavRouteStep> {
        match self.route {
            Some(route) => route
                .into_iter()
                .map(|step| state::NavRouteStep {
                    star_system: step.star_system,
                    system_address: step.system_address,
                    star_pos: step.star_pos,
                    star_class: step.star_class,
                })
                .collect(),
            None => Vec::new()
        }
    }
}