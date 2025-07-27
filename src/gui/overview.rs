mod ship;

use crate::event::JournalEvent;
use crate::gui::components::{details, header, sub_header};
use crate::gui::overview::ship::ship;
use crate::state::State;
use crate::theme::{DARK_GRAY, GRAY, ORANGE, WHITE};
use iced::border::radius;
use iced::widget::{column, container, row, scrollable, text, Column, Row};
use iced::{Border, Fill, Theme};
use std::f64;
use thousands::Separable;

pub fn overview(state: &State) -> Row<JournalEvent> {
    row![
        personal(state),
        column![
            route(state),
            location(state),

        ],
        ship(state)
    ]
}

fn location(state: &State) -> Column<JournalEvent> {
    let mut result = column![
        header("Location"),
        
        sub_header("System"),
        details("Government", &state.location.system_government),
        details("Economy", &state.location.system_economy),
        details("Population", state.location.population.to_string().separate_with_commas()),
        details("Security", &state.location.system_security),
        details("Allegiance", &state.location.system_allegiance),
    ]
        .padding(8);

    if state.location.powerplay_state.is_some() {
        result = result.push(sub_header("Powerplay"))
            .push(details("Controller", state.location.controlling_power.clone().unwrap_or_default()))
            .push(details("State", state.location.powerplay_state.clone().unwrap_or_default()))
            .push(details("Control", state.location.powerplay_state_control_progress.map(|x| x.to_string()).unwrap_or_default()))
            .push(details("Reinforcement", state.location.powerplay_state_reinforcement.map(|x| x.to_string()).unwrap_or_default()))
            .push(details("Undermining", state.location.powerplay_state_undermining.map(|x| x.to_string()).unwrap_or_default()));
    }

    if state.location.factions.is_some() {
        result = result.push(sub_header("Factions"));

        // todo: why do i have to clone this, i don't wanna
        for faction in state.location.factions.clone().unwrap() {
            result = result.push(
                row![
                    column![text(faction.name)].width(256),
                    column![text(faction.government).color(GRAY)].width(128),
                    column![].width(Fill),
                    column![text(format!("{:.2}%", faction.influence * 100.0)).color(GRAY)],
                ]
            );
        }
    }

    result
}

fn personal(state: &State) -> Column<JournalEvent> {
    column![
        header("Personal"),
        details("Suit Name", &state.suit_loadout.suit_name_localised),
        details("Loadout", &state.suit_loadout.loadout_name),
        details("Empire", state.reputation.empire.to_string()),
        details("Federation", state.reputation.federation.to_string()),
        details("Alliance", state.reputation.alliance.to_string()),
    ]
    .padding(8)
}

fn route(state: &State) -> Column<JournalEvent> {
    let title_column = column![header("Route")].padding(8);
    let mut items_column = column![].padding(8);

    for i in 0..state.nav_route.len() {
        let route_step = &state.nav_route[i];
        if i != 0 {
            let prev_step = &state.nav_route[i - 1];
            let distance = calculate_distance(&prev_step.star_pos, &route_step.star_pos);
            items_column = items_column.push(
                row![
                    container(row![
                        column![text(&route_step.star_system)],
                        column![].width(Fill),
                        column![text(format!("{:.2} ly", distance))]
                    ])
                    .style(route_step_style)
                    .padding(8)
                ]
                .padding(8)
                .width(Fill),
            );
        }
    }

    column![title_column, scrollable(items_column)].height(Fill)
}

fn calculate_distance(pos1: &Vec<f64>, pos2: &Vec<f64>) -> f64 {
    let dx = pos2[0] - pos1[0];
    let dy = pos2[1] - pos1[1];
    let dz = pos2[2] - pos1[2];
    f64::sqrt(dx * dx + dy * dy + dz * dz)
}

fn route_step_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(DARK_GRAY.into()),
        text_color: Some(WHITE),
        border: Border {
            width: 0.0,
            color: ORANGE,
            radius: radius(0),
        },
        shadow: Default::default(),
    }
}
