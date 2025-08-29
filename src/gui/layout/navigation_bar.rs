use crate::gui::Message;
use crate::image::gui::{COLLAPSE, EXPAND, SETTINGS};
use crate::state::{Screen, State};
use crate::theme::{style, GRAY, ORANGE, WHITE};
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, row, svg, Column, Row};
use iced::{Fill, Right, Theme};
use std::mem::discriminant;

pub fn navigation_bar(state: &State) -> Row<'_, Message> {
    // Right-side fullscreen toggle and settings button
    let fullscreen_icon = if state.layout.fullscreen { COLLAPSE } else { EXPAND };
    let fullscreen_button = button(
        svg(svg::Handle::from_memory(fullscreen_icon)).width(16).height(16).style(style::icon_button)
    )
        .on_press(Message::ToggleFullscreen)
        .style(default_style);

    let settings_button = button(
        svg(svg::Handle::from_memory(SETTINGS)).width(16).height(16).style(style::icon_button)
    )
        .on_press(Message::NavigateTo(Screen::Settings))
        .style(default_style);

    // Left: dynamic buttons for custom screens
    let mut custom_buttons: Vec<iced::Element<'_, Message>> = Vec::new();
    for (idx, scr) in state.layout.custom_screens.iter().enumerate() {
        let is_selected = matches!(state.active_screen, Screen::Commander) && state.layout.selected_custom_screen == idx;
        let style_fn = if is_selected { selected_style } else { default_style };
        custom_buttons.push(
            column![button(scr.name.as_ref()).on_press(Message::NavigateToCustomScreen(idx)).style(style_fn)].padding(5).into()
        );
    }

    row![
        row(custom_buttons),
        navigation_button(state, "MATERIALS", Screen::Materials),
        navigation_button(state, "SHIP LOCKER", Screen::ShipLocker),
        navigation_button(state, "MARKET", Screen::Market),
        navigation_button(state, "LOG", Screen::Messages),
        column![].width(Fill),

        // right-side buttons
        column![fullscreen_button].align_x(Right).padding([0, 4]),
        column![settings_button].align_x(Right).padding([0, 4])
    ]
}

fn navigation_button<'a>(state: &State, title: &'a str, screen: Screen) -> Column<'a, Message> {

    let style = if discriminant(&state.active_screen) == discriminant(&screen) { selected_style }
        else { default_style };

    let click_event = Message::NavigateTo(screen);
    column![button(title).on_press(click_event).style(style)].padding(5)
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