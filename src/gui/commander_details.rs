mod ship;

use crate::event::JournalEvent;
use crate::gui::commander_details::ship::ship;
use crate::gui::components::header;
use crate::state::State;
use iced::widget::{column, row, scrollable, text, Column, Row};
use iced::{Fill, Top};
use std::f64;

pub fn commander_details(state: &State) -> Row<JournalEvent> {
    row![
        legal_status(state),
        scrollable(location(state)),
        scrollable(ship(state)),
    ]
    .align_y(Top)
    .height(Fill)
}

fn legal_status(state: &State) -> Column<JournalEvent> {
    column![
        header("LEGAL"),
        row!["Legal State: ", text(&state.crime.legal_state)],
    ]
    .padding(8)
}


fn location(state: &State) -> Column<JournalEvent> {
    let mut column = column![
        header("ROUTE"),
    ]
        .padding(8);

    for i in 0..state.nav_route.len() {
        let route_step = &state.nav_route[i];
        if i == 0 {
            column = column.push(text(&route_step.star_system));
        } else {
            let prev_step = &state.nav_route[i - 1];
            let distance = calculate_distance(&prev_step.star_pos, &route_step.star_pos);
            column = column.push(text(format!("{} ({:.1} ly)", route_step.star_system, distance)));
        }
    }

    column
}
fn calculate_distance(pos1: &Vec<f64>, pos2: &Vec<f64>) -> f64 {
    let dx = pos2[0] - pos1[0];
    let dy = pos2[1] - pos1[1];
    let dz = pos2[2] - pos1[2];
    f64::sqrt(dx * dx + dy * dy + dz * dz)
}


