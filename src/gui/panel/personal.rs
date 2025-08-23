use crate::centered_row;
use crate::font::EUROSTILE;
use crate::gui::components::*;
use crate::gui::Message;
use crate::lookup::fdev_ids::Rank;
use crate::state::State;
use crate::theme::{style, GRAY, ORANGE, WHITE};
use iced::widget::{column, container, progress_bar, row, scrollable, text, Column};
use iced::{Element, Fill};

pub fn loadout(state: &State) -> Column<'_, Message> {
    column![
        sub_header("Suit"),
        details("Name", &state.suit_loadout.suit_name),
        details("Loadout", &state.suit_loadout.loadout_name),
        column(
            state.suit_loadout.suit_mods.iter().map(|mod_name| {
                details("Modification", mod_name).into()
            })
        ).padding(8),
        sub_header("Weapons: TODO"),
        // column(
        //     state.suit_loadout.modules.iter().map(|module| {
        //         column![
        //             details(&module.slot_name, &module.module_name),
        //             column(
        //                 module.weapon_mods.iter().map(|mod_name| {
        //                     details("Modification", mod_name).into()
        //                 })
        //             ).padding([0, 16])
        //         ].into()
        //     })
        // ).padding(8)
    ]
    .padding(8)
}

pub fn ranks(state: &State) -> Column<'_, Message> {
    column![
            row![
                rank("Combat Rank", state.rank.combat.as_str(), state.progress.combat, Rank::combat),
                rank("Explorer Rank", state.rank.explore.as_str(), state.progress.explore, Rank::exploration)
            ],
            row![
                rank("Trade Rank", state.rank.trade.as_str(), state.progress.trade, Rank::trading),
                rank("CQC Rank", state.rank.cqc.as_str(), state.progress.cqc, Rank::cqc)
            ],
            row![
                rank("Mercenary Rank", state.rank.mercenary.as_str(), state.progress.soldier, Rank::mercenary),
                rank("Exobiologist Rank", state.rank.exobiology.as_str(), state.progress.exobiologist, Rank::exobiologist)
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
                    Some(state.rank.federation.as_str()),
                    Some(state.progress.federation),
                    state.reputation.federation,
                    Some(Rank::federation)),
                superpower_rank(
                    "Empire",
                    Some(state.rank.empire.as_str()),
                    Some(state.progress.empire),
                    state.reputation.empire,
                    Some(Rank::empire))
            ],
        ].padding(8)
}

fn rank<'a>(title: &'a str, rank: &str, progress: u8, lookup: fn(&String) -> Option<&Rank>) -> Column<'a, Message> {
    let rank_name = match lookup(&rank.to_string()) {
        None => String::from("Unknown"),
        Some(title) => title.name.to_string()
    };

    column![
        container(
            column![
                centered_row![text(title).size(16).color(ORANGE)].padding(4),
                row![progress_bar(0f32..=100f32, progress as f32).height(8).style(style::progress_bar)].padding(4),
                centered_row![
                    row![
                        text(rank_name).size(16).color(WHITE)
                    ]
                ].padding(4)
            ]
        ).style(style::bordered)
    ]
        .padding(4)
}

fn superpower_rank<'a>(title: &'a str, rank: Option<&str>, progress: Option<u8>, reputation: f32, lookup: Option<fn(&String) -> Option<&Rank>>) -> Column<'a, Message> {
    let rank_name = match lookup {
        None => "".to_string(),
        Some(func) => {
            match rank {
                Some(r) => match func(&r.to_string()) {
                    None => String::from("Unknown"),
                    Some(title) => title.name.to_string()
                },
                None => String::from("Unknown")
            }
        }
    };

    column![
        container(
            column![
                centered_row![text(title).size(16).color(ORANGE)].padding(4),

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
                            text("Rank  ").size(16).color(GRAY),
                            text(rank_name).size(16).color(WHITE)
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
                        text("Reputation  ").size(16).color(GRAY),
                        text(
                            if reputation > 0.75 { "Allied" }
                            else if reputation > 0.35 { "Friendly" }
                            else if reputation > 0.15 { "Cordial" }
                            else if reputation > 0.0 { "Neutral" }
                            else { "Unfriendly" }
                        ).size(16).color(WHITE)
                        ]
                ].padding(4)
            ]
        ).style(style::bordered)
    ]
        .padding(4)
}

pub fn messages(state: &State) -> Column<'_, Message> {
    column![
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
                        row![text(&item.text).color(WHITE).font(EUROSTILE).size(16)]
                    ].width(Fill)
                })
                .map(Element::from)
        ))
        .anchor_bottom()
    ]
    .height(Fill)
    .padding(8)
}


pub fn claims(state: &State) -> Column<'_, Message> {

    if (state.bounties.len() == 0) && (state.combat_bonds.len() == 0) {
        return column![
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
        scrollable(column(all_claims.map(Element::from)))
    ].height(Fill)
}

pub fn missions(state: &State) -> Column<'_, Message> {

    if state.missions.len() == 0 {
        return column![
            empty_text("No Missions"),
        ]
    }

    column![
        scrollable(column(state.missions.iter().map(|m| {
            column![
                details(&m.faction, &m.name)
            ]
        }).map(Element::from)))
    ].height(Fill)
}