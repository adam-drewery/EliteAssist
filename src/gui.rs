mod overview;
mod header_bar;
mod materials;
mod navigation_bar;
mod ship_locker;
mod messages;
mod market;
mod components;

use crate::event::JournalEvent;
use crate::image::LOADING_PNG;
use crate::state::{ActiveScreen, State};
use crate::theme::{style, ORANGE};
use crate::{centered_column, centered_row, edsm};
use chrono::Utc;
use header_bar::header_bar;
use iced::widget::{column, progress_bar, row, svg, text};
use iced::{Bottom, Center, Element, Fill, Task};
use market::market;
use materials::materials;
use messages::messages;
use navigation_bar::navigation_bar;
use overview::overview;
use ship_locker::ship_locker;

#[derive(Clone, Debug)]
pub enum Message {
    NavigateTo(ActiveScreen),
    JournalEvent(JournalEvent),
    
    StationsQueried(edsm::Stations),
    SystemQueried(edsm::System),
    BodiesQueried(edsm::Bodies),
    FactionsQueried(edsm::Factions),
    TrafficQueried(edsm::Traffic),
    DeathsQueried(edsm::Deaths),
    
    JournalLoaded,
    Empty,
}

pub struct Gui;

impl Gui {
    pub fn view(state: &State) -> Element<'_, Message> {
        if state.commander_name.is_empty() {
            waiting_spinner()
        }
        else if !state.journal_loaded {
            loading_bar(state)
        }
        else {
            column![
                header_bar(state),
                match state.active_screen {
                    ActiveScreen::Commander => overview(state),
                    ActiveScreen::Materials => materials(state),
                    ActiveScreen::ShipLocker => ship_locker(state),
                    ActiveScreen::Market => market(state),
                    ActiveScreen::Messages => messages(state),
                }
                .height(Fill),
                navigation_bar(state).align_y(Bottom),
            ]
                .width(Fill)
                .padding(10)
                .into()
        }
    }

    pub fn update(state: &mut State, message: Message) -> Task<Message> {
        state.update_from(message)
    }
}

fn waiting_spinner() -> Element<'static, Message> {
    column![
                row![].height(Fill),
                row![
                    column![].width(Fill),
                    svg(svg::Handle::from_memory(LOADING_PNG))
                        .width(128)
                        .height(128),
                    column![].width(Fill)
                ],
                row![].height(Fill)
            ]
        .align_x(Center)
        .into()
}

fn loading_bar(state: &State) -> Element<'_, Message> {
    centered_column![
        centered_row![
            row![
                progress_bar(
                    state.first_message_timestamp as f32..=Utc::now().timestamp() as f32,
                    state.latest_message_timestamp as f32)
                .width(Fill)
                .style(style::progress_bar),
            ],
            row![
                column![text("Loading...").color(ORANGE).size(32)],
                column![].width(Fill),
                column![text(&state.latest_message_timestamp_formatted).color(ORANGE).size(32)]

            ]
        ]
    ]
    .into()
}