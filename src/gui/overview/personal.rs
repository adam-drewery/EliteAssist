use iced::{Element, Fill};
use crate::event::JournalEvent;
use crate::gui::components::{details, header};
use crate::state::State;
use iced::widget::{column, row, scrollable, text, Column};
use crate::theme::{GRAY, ORANGE, WHITE};

pub fn personal(state: &State) -> Column<JournalEvent> {

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

pub fn messages(state: &State) -> Column<JournalEvent> {
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


pub fn inventory(state: &State) -> Column<JournalEvent> {
    column![header("Inventory"),].height(Fill)

}

pub fn missions(state: &State) -> Column<JournalEvent> {
    column![header("Transactions"),].height(Fill)
}