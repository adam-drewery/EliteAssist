use crate::gui::components::*;
use crate::gui::Message;
use crate::state::State;
use crate::theme::{style, GRAY, ORANGE, WHITE};
use iced::widget::{column, container, progress_bar, row, scrollable, text, Column};
use iced::{Element, Fill};
use crate::centered_row;
use crate::lookup::fdev_ids::Rank;
use crate::font::eurocaps::FONT;

pub fn loadout(state: &State) -> Column<'_, Message> {
    column![
        column![
                header("Loadout"),
                details("Suit Name", &state.suit_loadout.suit_name),
                details("Loadout", &state.suit_loadout.loadout_name)
            ]
            .padding(8),
        column![
            header("Ranks"),
            row![
                rank("Combat Rank", state.rank.combat, state.progress.combat, Rank::combat),
                rank("Explorer Rank", state.rank.explore, state.progress.explore, Rank::exploration)
            ],
            row![
                rank("Trade Rank", state.rank.trade, state.progress.trade, Rank::trading),
                rank("CQC Rank", state.rank.cqc, state.progress.cqc, Rank::cqc)
            ],
            row![
                rank("Mercenary Rank", state.rank.soldier, state.progress.soldier, Rank::mercenary),
                rank("Exobiologist Rank", state.rank.exobiologist, state.progress.exobiologist, Rank::exobiologist)
            ],
            row![
                superpower_rank(
                    "Alliance",
                    None,
                    None,
                    state.reputation.alliance,
                    None),
                superpower_rank(
                    "Federation",
                    Some(state.rank.federation),
                    Some(state.progress.federation),
                    state.reputation.federation,
                    Some(Rank::federation)),
                superpower_rank(
                    "Empire",
                    Some(state.rank.empire),
                    Some(state.progress.empire),
                    state.reputation.empire,
                    Some(Rank::empire))
            ],
        ].padding(8)
    ]
}

fn rank(title: &str, rank: u64, progress: u64, lookup: fn(&String) -> Option<&Rank>) -> Column<'_, Message> {
    let rank_name = match lookup(&rank.to_string()) {
        None => String::from("Unknown"),
        Some(title) => title.name.clone()
    };

    column![
        container(
            column![
                centered_row![text(title).size(16).color(ORANGE).font(FONT)].padding(4),
                row![progress_bar(0f32..=100f32, progress as f32).height(8).style(style::progress_bar)].padding(4),
                centered_row![
                    row![
                        text(rank_name).size(16).color(WHITE).font(FONT),
                        text(format![" ({})", rank]).size(16).color(GRAY).font(FONT)
                    ]
                ].padding(4)
            ]
        ).style(style::bordered)
    ]
        .padding(4)
}

fn superpower_rank(title: &str, rank: Option<u64>, progress: Option<u64>, reputation: f64, lookup: Option<fn(&String) -> Option<&Rank>>) -> Column<'_, Message> {
    let rank_name = match lookup {
        None => "".to_string(),
        Some(func) => {
            match rank {
                Some(r) => match func(&r.to_string()) {
                    None => String::from("Unknown"),
                    Some(title) => title.name.clone()
                },
                None => String::from("Unknown")
            }
        }
    };

    column![
        container(
            column![
                centered_row![text(title).size(16).color(ORANGE).font(FONT)].padding(4),

                    if progress.is_some() {
                        row![
                        progress_bar(0f32..=100f32, progress.unwrap_or(0) as f32)
                        .height(8)
                        .style(style::progress_bar)
                    ]
                    } else {
                        row![].height(8).into()
                    }
                .padding(4),
                centered_row![
                    if rank.is_some() {
                        row![
                            text("Rank  ").size(16).color(GRAY).font(FONT),
                            text(rank_name).size(16).color(WHITE).font(FONT),
                            text(format![" ({})", rank.unwrap_or(0)]).size(16).color(GRAY).font(FONT)
                        ]
                    } else {
                        row![].height(29).into() // weird hack to get the text to line up
                    }
                ].padding(4),
                row![
                    progress_bar(0f32..=100f32, reputation as f32)
                    .height(8)
                    .style(style::progress_bar)
                ].padding(4),
                centered_row![
                    row![
                        text("Reputation  ").size(16).color(GRAY).font(FONT),
                        text(
                            if reputation > 0.75 { "Allied" }
                            else if reputation > 0.35 { "Friendly" }
                            else if reputation > 0.15 { "Cordial" }
                            else if reputation > 0.0 { "Neutral" }
                            else { "Unfriendly" }
                        ).size(16).color(WHITE).font(FONT)
                        ]
                ].padding(4)
            ]
        ).style(style::bordered)
    ]
        .padding(4)
}

pub fn messages(state: &State) -> Column<'_, Message> {
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
                    ].width(Fill)
                })
                .map(Element::from)
        ))
        .anchor_bottom()
    ]
    .height(256)
    .padding(8)
}


pub fn claims(state: &State) -> Column<'_, Message> {

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

pub fn missions(state: &State) -> Column<'_, Message> {

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