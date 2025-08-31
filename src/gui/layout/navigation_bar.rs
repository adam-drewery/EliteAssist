use crate::gui::Message;
use crate::image::gui::{COLLAPSE, EXPAND, SETTINGS};
use crate::state::{Screen, State};
use crate::theme::{style, GRAY, ORANGE, WHITE};
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, row, svg, text, Row};
use iced::{Color, Fill, Right, Theme};
use crate::message::Gui::{NavigateTo, NavigateToCustomScreen, ToggleFullscreen};

pub fn navigation_bar(state: &State) -> Row<'_, Message> {
    let fullscreen_icon = if state.layout.fullscreen {
        COLLAPSE
    } else {
        EXPAND
    };
    let fullscreen_button = button(
        svg(svg::Handle::from_memory(fullscreen_icon))
            .width(16)
            .height(16)
            .style(style::icon_button),
    )
    .on_press(Message::Gui(ToggleFullscreen))
    .style(default_style);

    let settings_button = button(
        svg(svg::Handle::from_memory(SETTINGS))
            .width(16)
            .height(16)
            .style(style::icon_button),
    )
    .on_press(Message::Gui(NavigateTo(Screen::Settings)))
    .style(default_style);

    let mut custom_buttons: Vec<iced::Element<'_, Message>> = Vec::new();
    for (idx, scr) in state.layout.custom_screens.iter().enumerate() {
        let is_selected = matches!(state.active_screen, Screen::Custom)
            && state.layout.selected_custom_screen == idx;
        let style_fn = if is_selected {
            selected_style
        } else {
            default_style
        };
        custom_buttons.push(
            column![
                button(scr.name.as_ref())
                    .on_press(Message::Gui(NavigateToCustomScreen(idx)))
                    .style(style_fn)
            ]
            .padding(5)
            .into(),
        );
    }

    let info_col: iced::Element<'_, Message> = if let Some(s) = &state.edsm_server_status {
        column![
            text("Server status").size(12).color(GRAY),
            text(s.message.as_ref())
                .size(12)
                .color(match s.status.as_ref() {
                    "success" => Color::from_rgb(0.0, 0.8, 0.0),
                    "warning" => ORANGE,
                    _ => Color::from_rgb(0.8, 0.0, 0.0),
                }),
        ]
        .align_x(Right)
        .padding([0, 4])
        .into()
    } else {
        column![].into()
    };

    row![
        row(custom_buttons),
        column![].width(Fill),
        info_col,
        column![fullscreen_button].align_x(Right).padding([0, 4]),
        column![settings_button].align_x(Right).padding([0, 4])
    ]

}

fn selected_style(_theme: &Theme, _status: Status) -> Style {
    Style {
        background: None,
        text_color: ORANGE,
        border: Default::default(),
        shadow: Default::default(),
    }
    .with_background(GRAY)
}

fn default_style(_theme: &Theme, _status: Status) -> Style {
    Style {
        background: None,
        text_color: WHITE,
        border: Default::default(),
        shadow: Default::default(),
    }
    .with_background(GRAY)
}
