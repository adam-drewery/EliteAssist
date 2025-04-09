use crate::event::JournalEvent;
use crate::state::{SlotType, State};
use crate::theme::styles::header_style;
use crate::theme::{GRAY, ORANGE, RED};
use iced::widget::{button, column, row, scrollable, text, Column, Row};
use iced::{Center, Element, Fill, Top};

pub fn commander_details(state: &State) -> Row<JournalEvent> {

    row![
        legal_status(state),
        location(state),
        scrollable(ship(state)),
    ]
    .align_y(Top)
    .height(Fill)
}

fn legal_status(state: &State) -> Column<JournalEvent> {
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

fn ship(state: &State) -> Column<JournalEvent> {
    let mut weapons: Vec<Element<JournalEvent>> = vec![];
    let mut utilities: Vec<Element<JournalEvent>> = vec![];
    //let mut core_internals: Vec<Element<JournalEvent>> = vec![];
    //let mut optional_internals: Vec<Element<JournalEvent>> = vec![];



    for m in state.ship_loadout.modules.iter() {
        match &m.slot {
            SlotType::Hardpoints { number: _, size } => {
                let row = row![
                        column![text(size).size(24).color(GRAY)],
                        column![text(m.rating).size(24).color(ORANGE)],
                        column![text(m.class).size(24).color(ORANGE)],
                        column![text(&m.name).size(16).color(ORANGE)],
                    ]
                    .padding(4)
                    .align_y(Center)
                    .into();

                if *size == 0 { utilities.push(row); } else { weapons.push(row); }
            }

            SlotType::CoreInternal(_) => {}

            SlotType::OptionalInternal(_) => {}

            SlotType::Cosmetic(_) => {}

            SlotType::Miscellaneous(_) => {}

            SlotType::Unknown(_) => {}
        }

    }

    column![
        button("SHIP").style(header_style).width(Fill),
        row![text("HARDPOINTS").color(GRAY).size(30)],
        column(weapons),
        row![text("UTILITIES").color(GRAY).size(30)],
        column(utilities),
        ]
        .padding(8)
}

fn location(state: &State) -> Column<JournalEvent> {
    column![ 
        button("LOCATION").style(header_style).width(Fill),
        text(&state.commander_name),
    ]
    .padding(8)
}