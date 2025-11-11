use crate::font::EUROSTILE;
use crate::gui::components::scroll_list;
use crate::gui::{pane, Message};
use crate::state::State;
use crate::theme::style;
use crate::theme::{BLUE, ORANGE, YELLOW};
use iced::widget::tooltip::Position;
use iced::widget::{column, row, text, tooltip, Column};
use iced::{Color, Element, Left};
use crate::state;

pub struct ShipLocker;

impl pane::Type for ShipLocker {
    fn title(&self) -> &'static str { "Ship Locker" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        column![row![
            locker_item_list("Items", &state.ship_locker.items),
            locker_item_list("Components", &state.ship_locker.components),
            locker_item_list("Data", &state.ship_locker.data),
            locker_item_list("Consumables", &state.ship_locker.consumables),
        ]]
        .into()
    }
}

fn locker_item_list<'a>(title: &'a str, items: &'a [state::ship::ShipLockerItem]) -> Column<'a, Message> {
    column![
        text(title).size(20).color(ORANGE),
        scroll_list(
            items
                .iter()
                .map(|item| {
                    let color = if item.for_mission { BLUE } else { Color::WHITE };

                    let content = row![
                        text(item.count).size(16).color(YELLOW).width(36),
                        text(item.name.as_ref())
                            .color(color)
                            .size(16)
                            .font(EUROSTILE),
                    ]
                    .padding(2);

                    if items.is_empty() || item.locations.is_empty() {
                        row![content]
                    } else {
                        row![
                            tooltip(
                                content,
                                column(
                                    item.locations
                                        .iter()
                                        .map(|loc| row![text(loc.as_ref()).size(16)].into())
                                        .collect::<Vec<Element<Message>>>()
                                ),
                                Position::FollowCursor,
                            )
                            .style(style::tooltip)
                        ]
                    }
                })
                .collect()
        )
    ]
    .align_x(Left)
}
