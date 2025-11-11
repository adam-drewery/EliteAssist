mod layout;
pub mod pane;
pub mod screen;
mod components;

use crate::gui::layout::header_bar;
use crate::gui::layout::navigation_bar;
use crate::image::LOADING_PNG;
use crate::message::Message;
use crate::state::{Screen, State};
use crate::theme::{style, ORANGE};
use crate::centered;
use chrono::Utc;
use iced::widget::{button, column, progress_bar, row, svg, text};
use iced::{Bottom, Center, Element, Fill, Task};

pub struct Gui;

impl Gui {

    pub fn view(state: &State) -> Element<'_, Message> {
        // Determine if any journal .log files exist in the currently configured/default directory
        let dir = crate::journal::get_journal_directory().unwrap_or_else(|_| crate::config::default_journal_dir());
        let has_logs = crate::journal::get_journal_paths(&dir).map(|v| !v.is_empty()).unwrap_or(false);

        if !has_logs {
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

fn waiting_spinner<'a>() -> Element<'a, Message> {
        let configured_dir = crate::journal::get_journal_directory()
            .unwrap_or_else(|_| crate::config::default_journal_dir());

        column![
                row![].height(Fill),
                row![
                    column![].width(Fill),
                    column![
                        svg(svg::Handle::from_memory(LOADING_PNG)).width(128).height(128),
                        text("The game logs directory is not found at:").color(ORANGE).size(16),
                        text(configured_dir.display().to_string()).color(ORANGE).size(16),
                        text("Please specify the directory manually").color(ORANGE).size(16),
                        button(text("Choose directory"))
                            .on_press(Message::Gui(crate::message::Gui::ChooseJournalDir))
                    ].align_x(Center),
                    column![].width(Fill)
                ],
                row![].height(Fill)
            ]
        .align_x(Center)
        .into()
}

fn loading_bar(state: &State) -> Element<'_, Message> {
    centered![
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
    .into()
}