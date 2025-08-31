use crate::gui::{pane, Message};
use crate::theme::{GRAY, ORANGE, style};
use iced::widget::{
    Column,
    Row,
    button,
    checkbox,
    column,
    row,
    scrollable,
    text,
    text_input,
    container,
    svg
};
use iced::{Element, Fill};
use iced::widget::svg::Handle;
use crate::gui::pane::Type;
use crate::image;
use crate::message::Gui::{
    AddCustomScreen,
    RemoveCustomScreen,
    RenameCustomScreen,
    SelectCustomScreen,
    TogglePane
};
use crate::state::State;

pub fn settings(state: &State) -> Row<'_, Message> {
    let screens_list: Column<'_, Message> = {
        let mut items: Vec<Element<'_, Message>> = Vec::new();
        for (idx, screen) in state.layout.custom_screens.iter().enumerate() {
            let is_selected = idx == state.layout.selected_custom_screen;
            items.push(
                row![
                    button(text(screen.name.as_ref()))
                        .on_press(Message::Gui(SelectCustomScreen(idx)))
                        .style(if is_selected { style::selected_button } else { style::button })
                        .width(Fill),
                ]
                .padding(4)
                .into(),
            );
        }

        column![
            row![
                column![text("Screens").size(24).color(ORANGE)],
                column![].width(Fill),
                column![button(svg(Handle::from_memory(image::gui::ADD)).height(16)).on_press(Message::Gui(AddCustomScreen))],
                column![].width(4),
                column![button(svg(Handle::from_memory(image::gui::REMOVE)).height(16)).on_press(Message::Gui(RemoveCustomScreen))]
            ],
            scrollable(column(items)).height(Fill).style(style::scrollable)
        ]
        .spacing(8)
    };

    // Right side: rename + pane toggles for selected
    let right_side: Column<'_, Message> = {
        let current_name = state.layout
            .custom_screens
            .get(state.layout.selected_custom_screen)
            .map(|s| s.name.as_ref())
            .unwrap_or("");

        let mut pane_items: Vec<Element<'_, Message>> = Vec::new();
        for id in pane::all().iter() {
            let checked = pane::is_enabled(*id, &state.layout);
            let id_copy: &'static dyn Type = *id;
            pane_items.push(
                checkbox(id.title(), checked)
                    .on_toggle(move |v| Message::Gui(TogglePane(id_copy.title().into(), v)))
                    .style(style::checkbox)
                    .into(),
            );
        }

        column![
            text("Screen Settings").size(24).color(ORANGE),
            text_input("Screen name", current_name)
                .on_input(|value: String| Message::Gui(RenameCustomScreen(value.into()))),
            text("Visible panes:").size(16).color(GRAY),
            scrollable(column(pane_items)).height(Fill).width(Fill).style(style::scrollable)
        ]
        .spacing(8)
    };

    row![
        column![
            row![].height(128),
            row![
                column![].width(Fill),
                column![container(screens_list).style(style::bordered).height(Fill).width(Fill).padding(8)].width(240),
                column![].width(8),
                column![container(right_side).style(style::bordered).height(Fill).width(Fill).padding(8)].width(240),
                column![].width(Fill)
            ],
            row![].height(128)
        ]
    ]
}
