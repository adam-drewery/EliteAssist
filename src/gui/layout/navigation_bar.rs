use crate::gui::Message;
use crate::state::{Screen, State};
use crate::theme::{GRAY, ORANGE, WHITE};
use iced::widget::button::{Status, Style};
use iced::widget::{button, row, Column, Row};
use iced::Theme;
use std::mem::discriminant;

pub fn navigation_bar(state: &State) -> Row<'_, Message> {
    row![
        navigation_button(state, "CMDR", Screen::Commander),
        navigation_button(state, "MATERIALS", Screen::Materials),
        navigation_button(state, "SHIP LOCKER", Screen::ShipLocker),
        navigation_button(state, "MARKET", Screen::Market),
        navigation_button(state, "LOG", Screen::Messages),
    ]
}

fn navigation_button<'a>(state: &State, title: &'a str, screen: Screen) -> Column<'a, Message> {

    let style = if discriminant(&state.active_screen) == discriminant(&screen) { selected_style }
        else { default_style };

    let click_event = Message::NavigateTo(screen);
    iced::widget::column![button(title).on_press(click_event).style(style)].padding(5)
}

fn selected_style(_theme: &Theme, _status: Status) -> Style {

    Style {
        background: None,
        text_color: ORANGE,
        border: Default::default(),
        shadow: Default::default(),
    }.with_background(GRAY)
}

fn default_style(_theme: &Theme, _status: Status) -> Style {
    Style {
        background: None,
        text_color: WHITE,
        border: Default::default(),
        shadow: Default::default(),
    }.with_background(GRAY)
}