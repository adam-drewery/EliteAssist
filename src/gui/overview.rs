mod ship;
mod location;
mod personal;

use crate::event::JournalEvent;
use crate::gui::components::header;
use crate::gui::overview::location::*;
use crate::gui::overview::personal::*;
use crate::gui::overview::ship::*;
use crate::state::State;
use crate::theme::{GRAY, ORANGE, WHITE};
use iced::widget::{column, row, scrollable, text, Column, Row};
use iced::{Element, Fill};

pub fn overview(state: &State) -> Row<JournalEvent> {
    row![
        column![
            personal(state),
            inventory(state),
            missions(state),
            messages(state)
        ],
        column![
            route(state),
            location(state),

        ],
        ship(state)
    ]
}

fn inventory(state: &State) -> Column<JournalEvent> {
    column![
        header("Inventory"),
    ]
    .height(Fill)
}

fn missions(state: &State) -> Column<JournalEvent> {
    column![
        header("Transactions"),
    ]
    .height(Fill)
}

fn messages(state: &State) -> Column<JournalEvent> {
    column![
        header("Messages"),
        scrollable(column(
                state
                    .messages
                    .iter()
                    .filter(|item| !item.from.is_empty())
                    .map(|item| {
                        column![
                            row![
                                column![text(&item.from).size(16).color(ORANGE)],
                                column![].width(12),
                                column![text(&item.time_display).size(12).color(GRAY)].padding(3),
                            ],
                            row![text(&item.text).color(WHITE).size(16)]
                        ]
                        .padding(4)
                    })
                    .map(Element::from)
            ))
            .anchor_bottom()
    ]
    .height(256)
}