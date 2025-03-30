use crate::event::Event;
use crate::state::{ActiveScreen, State};
use crate::theme::{BACKGROUND, ORANGE, WHITE};
use iced::widget::button::{Status, Style};
use iced::widget::{button, row, Column, Row};
use iced::Theme;
use std::mem::discriminant;

pub fn navigation_bar(state: &State) -> Row<Event> {
    row![
        navigation_button(state, "CMDR", ActiveScreen::Commander),
        navigation_button(state, "NAVIGATION", ActiveScreen::Navigation),
        navigation_button(state, "SHIP LOCKER", ActiveScreen::ShipLocker),
        navigation_button(state, "MARKET", ActiveScreen::Market),
    ]
}

fn navigation_button<'a>(state: &State, title: &'a str, screen: ActiveScreen) -> Column<'a, Event> {
    
    let style = if discriminant(&state.active_screen) == discriminant(&screen) { selected_style } 
        else { default_style };
    
    let click_event = Event::NavigateTo(screen);
    iced::widget::column![button(title).on_press(click_event).style(style)].padding(5)
}

// I'm sure there must be a better way to do this.

fn selected_style(_theme: &Theme, _status: Status) -> Style {
    
    Style {
        background: None,
        text_color: ORANGE,
        border: Default::default(),
        shadow: Default::default(),
    }.with_background(BACKGROUND)
}

fn default_style(_theme: &Theme, _status: Status) -> Style {
    Style {
        background: None,
        text_color: WHITE,
        border: Default::default(),
        shadow: Default::default(),
    }.with_background(BACKGROUND)
}