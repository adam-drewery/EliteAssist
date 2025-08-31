use crate::font::EUROSTILE;
use crate::gui::components::*;
use crate::gui::Message;
use crate::state::State;
use crate::theme::GRAY;
use iced::widget::{column, row, text, Column};
use iced::{Element, Fill};
use thousands::Separable;

pub struct LocationPane;

impl crate::gui::pane::PaneType for LocationPane {
    
    fn title(&self) -> &'static str { "Location" }
    
    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        column![
            row![system(state), powerplay(state)],
            station_summary(state),
            factions(state),
            station_economies(state),
            stations(state),
            bodies(state),
            nearby_systems(state),
            activity(state)
        ]
        .into()
    }
}

fn factions(state: &State) -> Column<'_, Message> {
    
    if state.location.factions.is_empty() { return column![]; }

    let mut result = column![sub_header("Factions")];

    for faction in &state.location.factions {
        let active_state_names: Vec<&str> = faction
            .active_states
            .iter()
            .map(|s| s.state.as_ref())
            .collect();

        let recovering_state_names: Vec<&str> = faction
            .recovering_states
            .iter()
            .map(|s| s.state.as_ref())
            .collect();

        let header = format!(
            "{} | {} | {}",
            faction.allegiance,
            faction.government,
            active_state_names.join(" | ")
        );

        let extras = format!(
            "State: {} | Influence: {:.2}% | Happiness: {} | Rep: {:.2}",
            faction.faction_state,
            faction.influence * 100.0,
            faction.happiness,
            faction.my_reputation
        );

        result = result.push(column![
            row![
                column![text(faction.name.as_ref()).font(EUROSTILE)],
                column![].width(Fill),
                column![text(header).color(GRAY).font(EUROSTILE)],
            ],
            if !recovering_state_names.is_empty() || !extras.is_empty() {
                row![
                    column![].width(Fill),
                    column![text(extras).color(GRAY).font(EUROSTILE)],
                ]
            } else { row![] },
            if !recovering_state_names.is_empty() {
                row![
                    column![].width(Fill),
                    column![text(format!("Recovering: {}", recovering_state_names.join(", "))).color(GRAY).font(EUROSTILE)],
                ]
            } else { row![] }
        ]);
    }

    result
}

fn powerplay(state: &State) -> Column<'_, Message> {
    
    if state.location.powerplay_state.is_none() { return column![]; }

    column![
        sub_header("Powerplay"),
        details(
            "Controller",
            state.location.controlling_power.clone().unwrap_or_default()
        ),
        details(
            "Control Progress",
            state
                .location
                .powerplay_state_control_progress
                .map(|x| format!("{:.2}%", x * 100.0))
                .unwrap_or_default()
        ),
        details(
            "Conflict Progress",
            state
                .location
                .powerplay_state_conflict_progress
                .map(|x| format!("{:.2}%", x * 100.0))
                .unwrap_or_default()
        ),
        details(
            "State",
            state.location.powerplay_state.clone().unwrap_or_default()
        ),
        details(
            "Reinforcement",
            state
                .location
                .powerplay_state_reinforcement
                .map(|x| x.to_string())
                .unwrap_or_default()
        ),
        details(
            "Undermining",
            state
                .location
                .powerplay_state_undermining
                .map(|x| x.to_string())
                .unwrap_or_default()
        ),
        details(
            "Powers",
            state
                .location
                .powers
                .as_ref()
                .map(|ps| ps.iter().map(|p| p.as_ref()).collect::<Vec<&str>>().join(", "))
                .unwrap_or_default()
        )
    ]
}

fn system(state: &State) -> Column<'_, Message> {
    let system_faction = state
        .location
        .system_faction
        .as_ref()
        .map(|sf| match &sf.faction_state {
            Some(st) => format!("{} ({})", sf.name.as_ref(), st.as_ref()),
            None => sf.name.to_string(),
        })
        .unwrap_or_default();

    column![
        sub_header("System"),
        details("Name", state.location.star_system.as_ref()),
        details("Address", state.location.system_address.to_string()),
        details("Government", state.location.system_government.as_ref()),
        details("Economy", state.location.system_economy.as_ref()),
        details("Second Economy", state.location.system_second_economy.as_ref()),
        details(
            "Population",
            state.location.population.to_string().separate_with_commas()
        ),
        details(
            "Security",
            &state.location.system_security.replace(" Security", "")
        ),
        details("Allegiance", state.location.system_allegiance.as_ref()),
        details("Faction", system_faction),
    ]
}

fn station_summary(state: &State) -> Column<'_, Message> {
    let docked = if state.location.docked { "Yes" } else { "No" };
    let station_faction = state
        .location
        .station_faction
        .as_ref()
        .map(|sf| match &sf.faction_state {
            Some(st) => format!("{} ({})", sf.name.as_ref(), st.as_ref()),
            None => sf.name.to_string(),
        })
        .unwrap_or_default();
    let services = state
        .location
        .station_services
        .as_ref()
        .map(|ss| ss.iter().map(|s| s.as_ref()).collect::<Vec<&str>>().join(", "))
        .unwrap_or_default();

    let coords = if state.location.star_pos.len() == 3 {
        format!(
            "{:.2}, {:.2}, {:.2}",
            state.location.star_pos[0],
            state.location.star_pos[1],
            state.location.star_pos[2]
        )
    } else {
        String::new()
    };

    column![
        sub_header("Status"),
        details("Docked", docked),
        details("Station", state.location.station_name.clone().unwrap_or_default()),
        details("Station Type", state.location.station_type.clone().unwrap_or_default()),
        details("Station Faction", station_faction),
        details("Station Government", state.location.station_government.clone().unwrap_or_default()),
        details("Station Services", services),
        details("Station Economy", state.location.station_economy.clone().unwrap_or_default()),
        details("Taxi", state.location.taxi.map(|b| if b { "Yes" } else { "No" }).unwrap_or_default()),
        details("Multicrew", state.location.multicrew.map(|b| if b { "Yes" } else { "No" }).unwrap_or_default()),
        details("Body", state.location.body.as_ref()),
        details("Body ID", state.location.body_id.to_string()),
        details("Body Type", state.location.body_type.as_ref()),
        details("Coordinates", coords),
    ]
}

fn station_economies(state: &State) -> Column<'_, Message> {
    if state.location.station_economies.is_empty() { return column![]; }
    let mut col = column![sub_header("Station Economies")];
    for e in &state.location.station_economies {
        col = col.push(details(e.name.as_ref(), format!("{:.2}%", e.proportion * 100.0)));
    }
    col
}

fn stations(state: &State) -> Column<'_, Message> {
    if state.location.stations.is_empty() { return column![]; }
    let mut col = column![sub_header("Stations")];
    for s in &state.location.stations {
        let services = s.other_services.iter().map(|x| x.as_ref()).collect::<Vec<&str>>();
        let services_str = if services.is_empty() { String::new() } else { services.join(", ") };
        let body_str = s.body.as_ref().map(|b| {
            let lat = b.latitude.map(|v| format!("{:.4}", v)).unwrap_or_default();
            let lon = b.longitude.map(|v| format!("{:.4}", v)).unwrap_or_default();
            format!("{} (#{}) {}{}{}{}{}",
                b.name,
                b.id,
                if lat.is_empty() && lon.is_empty() { "" } else { "[" },
                lat,
                if lat.is_empty() || lon.is_empty() { "" } else { ", " },
                lon,
                if lat.is_empty() && lon.is_empty() { "" } else { "]" }
            )
        }).unwrap_or_default();

        let updated = format!(
            "Info: {} | Market: {} | Shipyard: {} | Outfitting: {}",
            s.update_time.information,
            s.update_time.market.clone().unwrap_or_default(),
            s.update_time.shipyard.clone().unwrap_or_default(),
            s.update_time.outfitting.clone().unwrap_or_default()
        );

        col = col.push(column![
            sub_header(s.name.as_ref()),
            details("Type", s.type_field.as_ref()),
            details("ID", s.id.to_string()),
            details("Market ID", s.market_id.to_string()),
            details("Body", body_str),
            details("Distance to Arrival", format!("{:.1} ls", s.distance_to_arrival)),
            details("Allegiance", s.allegiance.as_ref()),
            details("Government", s.government.as_ref()),
            details("Economy", s.economy.as_ref()),
            details("Second Economy", s.second_economy.clone().unwrap_or_default()),
            details("Has Market", if s.have_market { "Yes" } else { "No" }),
            details("Has Shipyard", if s.have_shipyard { "Yes" } else { "No" }),
            details("Has Outfitting", if s.have_outfitting { "Yes" } else { "No" }),
            details("Other Services", services_str),
            details("Controlling Faction", s.controlling_faction.clone().unwrap_or_default()),
            details("Updated", updated),
        ]);
    }
    col
}

fn bodies(state: &State) -> Column<'_, Message> {
    if state.location.known_bodies.is_empty() { return column![]; }
    let mut col = column![sub_header("Bodies")];
    for b in &state.location.known_bodies {
        col = col.push(column![
            sub_header(b.name.as_ref()),
            details("Type", b.type_field.as_ref()),
            details("Subtype", b.sub_type.as_ref()),
            details("Distance to Arrival", format!("{:.1} ls", b.distance_to_arrival)),
            details("Main Star", if b.is_main_star { "Yes" } else { "No" }),
            details("Scoopable", if b.is_scoopable { "Yes" } else { "No" }),
        ]);
    }
    col
}

fn nearby_systems(state: &State) -> Column<'_, Message> {
    if state.location.nearby_systems.is_empty() { return column![]; }
    let mut col = column![sub_header("Nearby Systems")];
    for sys in &state.location.nearby_systems {
        col = col.push(details(sys.name.as_ref(), format!("#{}", sys.address)));
    }
    col
}

fn activity(state: &State) -> Column<'_, Message> {
    let mut col = column![sub_header("Activity")];
    if let Some(t) = &state.location.traffic {
        col = col.push(details("Traffic (Day/Week/Total)", format!("{}/{}/{}", t.day, t.week, t.total)));
    }
    if let Some(d) = &state.location.deaths {
        col = col.push(details("Deaths (Day/Week/Total)", format!("{}/{}/{}", d.day, d.week, d.total)));
    }
    col
}