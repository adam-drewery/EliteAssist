use crate::event::Event;
use crate::state::State;
use crate::theme::styles::header_style;
use crate::theme::{GRAY, RED};
use iced::widget::{button, column, row, text, Column, Row};
use iced::{Fill, Top};

pub fn commander_details(state: &State) -> Row<Event> {

    row![
        legal_status(state),
        location(state),
        ship(state),
    ]
    .align_y(Top)
    .height(Fill)
}

fn legal_status(state: &State) -> Column<Event> {
    column![ 
        button("LEGAL").style(header_style).width(Fill),
        row!["Legal State: ", text(&state.crime.legal_state)],
        if state.crime.active_fine { text("Active Fine").color(RED).width(Fill) }
            else { text("No Active Fine").color(GRAY).width(Fill) },
        if state.crime.wanted { text("Wanted").color(RED).width(Fill) }
            else { text("Not Wanted").color(GRAY).width(Fill) },
    ]
    .padding(8)
}

fn ship(state: &State) -> Column<Event> {
    column![
        
        
        
        // hardpoints
        // core internal
        // utility mounts
        // optional internal
        
        // fighter bay
        // multicrew
        // pad requirement
        
        button("SHIP").style(header_style).width(Fill),
        
        row![text("HARDPOINTS").color(GRAY).size(30)],
        
        column(
            state.ship_loadout.modules.iter().map(|m| { 
                row![text(&m.item)].into()
            })
        ),
        
        row![text("CORE INTERNAL").color(GRAY).size(30)],
        row![text("UTILITY MOUNTS").color(GRAY).size(30)],
        row![text("OPTIONAL INTERNAL").color(GRAY).size(30)],
        row![text("FIGHTER BAY").color(GRAY).size(30)]
        // text(&state.ship_loadout.hull_health),
        // text(&state.ship_loadout.ship_name),
        // text(&state.ship_loadout.ship_ident),
    ]
    .padding(8)
}

fn location(state: &State) -> Column<Event> {
    column![ 
        button("LOCATION").style(header_style).width(Fill),
        text(&state.commander_name),
    ]
    .padding(8)
}