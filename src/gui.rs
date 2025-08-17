mod overview;
mod header_bar;
mod materials;
mod navigation_bar;
mod ship_locker;
mod messages;
mod market;
mod components;

use crate::edsm::EdsmSystem;
use crate::event::JournalEvent;
use crate::image::LOADING_PNG;
use crate::state::{ActiveScreen, State};
use header_bar::header_bar;
use iced::widget::{column, row, svg};
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
    SystemQueried(EdsmSystem),
    JournalLoaded,
    Empty,
}

pub struct Gui;

impl Gui {
    pub fn view(state: &State) -> Element<'_, Message> {
        if state.commander_name.is_empty() {
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
