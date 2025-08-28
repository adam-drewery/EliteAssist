mod layout;
mod panel;
mod screen;
mod components;

use crate::font::EUROSTILE;
use crate::gui::layout::header_bar;
use crate::gui::layout::navigation_bar;
use crate::gui::screen::market;
use crate::gui::screen::materials;
use crate::gui::screen::messages;
use crate::gui::screen::overview;
use crate::gui::screen::ship_locker;
use crate::image::LOADING_PNG;
use crate::journal::event::Event;
use crate::state::{pane, Screen, State};
use crate::theme::{style, ORANGE};
use crate::{ardent, centered_column, centered_row, edsm};
use chrono::Utc;
use iced::widget::{column, pane_grid, progress_bar, row, svg, text};
use iced::{Bottom, Center, Element, Fill, Task};

#[derive(Clone, Debug)]
pub enum Message {
    NavigateTo(Screen),
    JournalEvent(Event),
    
    StationsQueried(edsm::Stations),
    NearbySystemsQueried(Vec<ardent::NearbySystem>),
    BodiesQueried(edsm::Bodies),
    TrafficQueried(edsm::Traffic),
    DeathsQueried(edsm::Deaths),
    
    // Pane grid interactions on the Overview screen
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),

    // Settings menu / pane toggles
    ShowSettingsMenu(bool),
    TogglePane(pane::Type, bool),

    // Window controls
    ToggleFullscreen,
    ToggleFullscreenWithId(Option<iced::window::Id>),

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
            main_layout(state)
        }
    }

    pub fn update(state: &mut State, message: Message) -> Task<Message> {
        state.update_from(message)
    }
}

fn main_layout(state: &State) -> Element<'_, Message> {
    column![
        header_bar(state),
        match state.active_screen {
            Screen::Commander => overview(state),
            Screen::Materials => materials(state),
            Screen::ShipLocker => ship_locker(state),
            Screen::Market => market(state),
            Screen::Messages => messages(state),
        }
        .height(Fill),
        navigation_bar(state).align_y(Bottom),
    ]
    .width(Fill)
    .padding(10)
    .into()
}

fn waiting_spinner<'a>() -> Element<'a, Message> {
        column![
                row![].height(Fill),
                row![
                    column![].width(Fill),
                    column![
                        svg(svg::Handle::from_memory(LOADING_PNG)).width(128).height(128),
                        text("Waiting for Journal Files...").color(ORANGE).size(32),
                        text("todo: make the loading spinner animated").font(EUROSTILE).color(ORANGE).size(12),
                    ].align_x(Center),
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
                column![text(state.latest_message_timestamp_formatted.as_ref()).color(ORANGE).size(32)]

            ]
        ]
    ]
    .into()
}