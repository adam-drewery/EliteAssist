mod layout;
pub mod pane;
pub mod screen;
mod components;

use crate::font::EUROSTILE;
use crate::gui::layout::header_bar;
use crate::gui::layout::navigation_bar;
use crate::image::LOADING_PNG;
use crate::message::Message;
use crate::message::Gui as GuiMsg;
use crate::state::{Screen, State, DataSource};
use crate::theme::{style, ORANGE};
use crate::{centered_column, centered_row};
use chrono::Utc;
use iced::widget::{button, column, progress_bar, row, svg, text};
use iced::{Bottom, Center, Element, Fill, Task};

pub struct Gui;

impl Gui {

    pub fn view(state: &State) -> Element<'_, Message> {
        match state.data_source {
            DataSource::Unselected => start_screen(state),
            _ => {
                if !state.journal_loaded {
                    loading_bar(state)
                } else {
                    main_layout(state)
                }
            }
        }
    }

    pub fn update(state: &mut State, message: Message) -> Task<Message> {
        message.update(state)
    }
}

fn main_layout(state: &State) -> Element<'_, Message> {
    column![
        header_bar(state),
        match state.active_screen {
            Screen::Custom => screen::custom(state),
            Screen::Settings => screen::settings(state),
        }
        .height(Fill),
        navigation_bar(state).align_y(Bottom),
    ]
    .width(Fill)
    .padding(10)
    .into()
}

fn start_screen(state: &State) -> Element<'_, Message> {
    let local_btn = button(text("Local Journals").size(24))
        .on_press(Message::Gui(GuiMsg::ChooseDataSource(DataSource::Local)));

    let capi_btn = button(text("Companion API").size(24))
        .on_press(Message::Gui(GuiMsg::ChooseDataSource(DataSource::Capi)));

    let status = if state.auth_in_progress {
        text("Authenticating in browser...").color(ORANGE)
    } else if let Some(err) = &state.auth_error {
        text(format!("Auth failed: {}", err)).color(ORANGE)
    } else if state.capi_enabled {
        text("Companion API token found; ready.").color(ORANGE)
    } else {
        text("")
    };

    column![
        row![].height(Fill),
        row![
            column![].width(Fill),
            column![
                svg(svg::Handle::from_memory(LOADING_PNG)).width(128).height(128),
                text("Choose data source").color(ORANGE).size(32),
                row![local_btn, capi_btn].spacing(20),
                status,
                text("Note: Frontier OAuth requires a browser; configure FRONTIER_CLIENT_ID env var.")
                    .font(EUROSTILE).size(12).color(ORANGE),
            ].align_x(Center).spacing(16),
            column![].width(Fill)
        ],
        row![].height(Fill)
    ]
    .align_x(Center)
    .into()
}

fn loading_bar(state: &State) -> Element<'_, Message> {
    // Optional status line for CAPI auth feedback so users see activity immediately
    let status_line: Element<'_, Message> = if matches!(state.data_source, DataSource::Capi) {
        if state.auth_in_progress {
            text("Authenticating with Frontier in your browser...").color(ORANGE).size(16).into()
        } else if let Some(err) = &state.auth_error {
            text(format!("Auth failed: {}", err)).color(ORANGE).size(16).into()
        } else if !state.capi_enabled && !state.journal_loaded {
            text("Waiting for Companion API authentication. Ensure FRONTIER_CLIENT_ID is set.")
                .color(ORANGE)
                .size(16)
                .into()
        } else {
            text("").into()
        }
    } else { text("").into() };

    centered_column![
        centered_row![
            row![status_line],
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