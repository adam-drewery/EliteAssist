use crate::gui::Message;
use crate::image::SETTINGS;
use crate::state::{PanelType, Screen, State};
use crate::theme::{style, GRAY, ORANGE, WHITE};
use iced::widget::button::{Status, Style};
use iced::widget::{button, checkbox, column, row, svg, Column, Row};
use iced::{Fill, Theme};
use std::mem::discriminant;

pub fn navigation_bar(state: &State) -> Row<'_, Message> {
    // Right-side settings button and optional menu
    let settings_button = button(
        svg(svg::Handle::from_memory(SETTINGS)).width(16).height(16).style(style::icon_button)
    )
        .on_press(Message::ShowSettingsMenu(!state.show_settings_menu))
        .style(default_style);

    let settings_menu: Column<'_, Message> = if state.show_settings_menu {
        // Build a list of checkboxes for each available panel
        let mut items: Vec<iced::Element<'_, Message>> = Vec::new();
        for panel in PanelType::all().iter() {
            let checked = state.is_panel_enabled(panel);
            let p = panel.clone();
            let cb = checkbox(panel.title(), checked)
                .on_toggle(move |v| Message::TogglePanel(p.clone(), v));
            items.push(cb.into());
        }
        column(items)
            .padding(6)
            .spacing(4)
    } else {
        column![]
    };

    row![
        navigation_button(state, "CMDR", Screen::Commander),
        navigation_button(state, "MATERIALS", Screen::Materials),
        navigation_button(state, "SHIP LOCKER", Screen::ShipLocker),
        navigation_button(state, "MARKET", Screen::Market),
        navigation_button(state, "LOG", Screen::Messages),
        column![].width(Fill),
        column![settings_button, settings_menu]
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