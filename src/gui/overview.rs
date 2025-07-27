mod ship;

use crate::event::JournalEvent;
use crate::gui::components::{details, header};
use crate::gui::overview::ship::ship;
use crate::state::State;
use crate::theme::{DARK_GRAY, ORANGE, WHITE};
use iced::border::radius;
use iced::widget::{column, container, row, scrollable, text, Column, Row};
use iced::{Border, Fill, Theme};
use std::f64;

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
    column![
        header("CURRENT LOCATION"),
        details("LEGAL STATE", state.crime.legal_state.to_uppercase()),
    ]
        .padding(8)
}

fn personal(state: &State) -> Column<JournalEvent> {
    column![
        header("PERSONAL"),
        details("SUIT NAME", state.suit_loadout.suit_name_localised.to_uppercase()),
        details("LOADOUT", state.suit_loadout.loadout_name.to_uppercase()),
        details("EMPIRE", state.reputation.empire.to_string()),
        details("FEDERATION", state.reputation.federation.to_string()),
        details("ALLIANCE", state.reputation.alliance.to_string()),
    ]
    .padding(8)
}

fn route(state: &State) -> Column<JournalEvent> {
    let title_column = column![header("ROUTE")].padding(8);
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
                        column![text(format!("{:.2} ly)", distance))]
                    ])
                    .style(route_step_style)
                    .padding(6)
                ]
                .padding(6)
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
