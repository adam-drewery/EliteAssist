use iced::{Element, Fill};
use crate::event::JournalEvent;
use crate::gui::components::*;
use crate::state::State;
use iced::widget::{column, row, scrollable, text, Column};
use crate::theme::{GRAY, ORANGE, WHITE};

pub fn personal(state: &State) -> Column<JournalEvent> {

    column![
        header("Personal"),
        details("Suit Name", &state.suit_loadout.suit_name),
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


pub fn claims(state: &State) -> Column<JournalEvent> {

    if (state.bounties.len() == 0) && (state.combat_bonds.len() == 0) {
        return column![
            header("Claims"),
            empty_text("No Claims"),
        ].height(Fill)
    }

    let all_claims = state.bounties.iter().map(|m| {
        details(&m.0, format!["{} CR", &m.1])
    }).chain(
        state.combat_bonds.iter().map(|m| {
            details(&m.0, format!["{} CR", &m.1])
        })
    );

    column![
        header("Claims"),
        scrollable(column(all_claims.map(Element::from)))
    ].height(Fill)
}

pub fn missions(state: &State) -> Column<JournalEvent> {

    if state.missions.len() == 0 {
        return column![
            header("Missions"),
            empty_text("No Missions"),
        ]
    }

    column![
        header("Missions"),
        scrollable(column(state.missions.iter().map(|m| {
            column![
                details(&m.faction, &m.name)
            ]
        }).map(Element::from)))
    ].height(Fill)
}