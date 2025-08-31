use iced::Element;
use crate::centered_row;
use crate::gui::Message;
use crate::lookup::fdev_ids::Rank;
use crate::state::State;
use crate::theme::{style, GRAY, ORANGE, WHITE};
use iced::widget::{column, container, progress_bar, row, text, scrollable, Column};

pub struct RanksPane;

impl crate::gui::pane::PaneType for RanksPane {

    fn title(&self) -> &'static str { "Ranks" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        let content = column![
            row![
                rank(
                    "Combat Rank",
                    state.rank.combat,
                    state.progress.combat,
                    Rank::combat
                ),
                rank(
                    "Explorer Rank",
                    state.rank.explore,
                    state.progress.explore,
                    Rank::exploration
                )
            ],
            row![
                rank(
                    "Trade Rank",
                    state.rank.trade,
                    state.progress.trade,
                    Rank::trading
                ),
                rank("CQC Rank", state.rank.cqc, state.progress.cqc, Rank::cqc)
            ],
            row![
                rank(
                    "Mercenary Rank",
                    state.rank.soldier,
                    state.progress.soldier,
                    Rank::mercenary
                ),
                rank(
                    "Exobiologist Rank",
                    state.rank.exobiologist,
                    state.progress.exobiologist,
                    Rank::exobiologist
                )
            ],
            row![
                superpower_rank("Alliance", None, None, state.reputation.alliance, None),
                superpower_rank(
                    "Federation",
                    Some(state.rank.federation),
                    Some(state.progress.federation),
                    state.reputation.federation,
                    Some(Rank::federation)
                ),
                superpower_rank(
                    "Empire",
                    Some(state.rank.empire),
                    Some(state.progress.empire),
                    state.reputation.empire,
                    Some(Rank::empire)
                )
            ],
        ];

        let content = row![
            content,
            column![].width(16)
        ];

        column![
            scrollable(content)
                .style(style::scrollable)
        ]
        .into()
    }
}

fn rank(
    title: &str,
    rank: u8,
    progress: u8,
    lookup: fn(&str) -> Option<&Rank>,
) -> Column<'_, Message> {
    let rank_name = match lookup(&rank.to_string()) {
        None => String::from("Unknown"),
        Some(title) => title.name.to_string(),
    };

    iced::widget::column![
        container(column![
            centered_row![text(title).size(16).color(ORANGE)].padding(4),
            row![
                progress_bar(0f32..=100f32, progress as f32)
                    .height(8)
                    .style(style::progress_bar)
            ]
            .padding(4),
            centered_row![row![
                text(rank_name).size(16).color(WHITE),
                text(format![" ({})", rank]).size(16).color(GRAY)
            ]]
            .padding(4)
        ])
        .style(style::bordered)
    ]
    .padding(4)
}

fn superpower_rank(
    title: &str,
    rank: Option<u8>,
    progress: Option<u8>,
    reputation: f64,
    lookup: Option<fn(&str) -> Option<&Rank>>,
) -> Column<'_, Message> {
    let rank_name = match lookup {
        None => "".to_string(),
        Some(func) => match rank {
            Some(r) => match func(&r.to_string()) {
                None => String::from("Unknown"),
                Some(title) => title.name.to_string(),
            },
            None => String::from("Unknown"),
        },
    };

    iced::widget::column![
        container(column![
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
            centered_row![if rank.is_some() {
                row![
                    text("Rank  ").size(16).color(GRAY),
                    text(rank_name).size(16).color(WHITE),
                    text(format![" ({})", rank.unwrap_or(0)])
                        .size(16)
                        .color(GRAY)
                ]
            } else {
                row![].height(29).into() // weird hack to get the text to line up
            }]
            .padding(4),
            row![
                progress_bar(0f32..=100f32, reputation as f32)
                    .height(8)
                    .style(style::progress_bar)
            ]
            .padding(4),
            centered_row![row![
                text("Reputation  ").size(16).color(GRAY),
                text(if reputation > 0.75 {
                    "Allied"
                } else if reputation > 0.35 {
                    "Friendly"
                } else if reputation > 0.15 {
                    "Cordial"
                } else if reputation > 0.0 {
                    "Neutral"
                } else {
                    "Unfriendly"
                })
                .size(16)
                .color(WHITE)
            ]]
            .padding(4)
        ])
        .style(style::bordered)
    ]
    .padding(4)
}
