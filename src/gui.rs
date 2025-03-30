use crate::controls::*;
use crate::events::EliteEvent;
use crate::state::State;
use crate::color::*;
use iced::widget::{column, container, row, text};
use iced::{Element, Fill, Left, Right, Top};

pub struct Gui;

impl Gui {
    pub fn view(state: &State) -> Element<EliteEvent> {
        container(column![
            row![
                column![
                    text(&state.commander_name)
                        .size(30)
                        .color(ORANGE),
                    text(&state.credits).size(30),
                ]
                .width(Fill)
                .align_x(Left),
                column![
                    text(&state.current_system).size(30),
                    text(&state.current_body).size(30),
                ]
                .width(Fill)
                .align_x(Right),
            ],
            row![
                inventory_list("ITEMS", state.ship_locker.items.clone()),
                inventory_list("COMPONENTS", state.ship_locker.components.clone()),
                inventory_list("DATA", state.ship_locker.data.clone()),
                inventory_list("CONSUMABLES", state.ship_locker.consumables.clone()),
            ]
            .align_y(Top)
            .height(Fill)
        ])
        .padding(10)
        .center_x(Fill)
        .into()
    }

    pub fn update(state: &mut State, message: EliteEvent) {
        state.update_from(message);
    }
}
