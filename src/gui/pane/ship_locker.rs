use crate::font::EUROSTILE;
use crate::gui::Message;
use crate::state::{ShipLockerItem, State};
use crate::theme::style;
use crate::theme::{BLUE, ORANGE, YELLOW};
use iced::widget::tooltip::Position;
use iced::widget::{column, row, scrollable, text, tooltip, Column};
use iced::{Color, Fill, Left};

pub struct ShipLockerPane;

impl crate::gui::pane::PaneType for ShipLockerPane {
    fn id(&self) -> &'static str {
        "ship_locker"
    }
    fn title(&self) -> &'static str {
        "Ship Locker"
    }
    fn render<'a>(&self, state: &'a State) -> iced::Element<'a, Message> {
        column![row![
            locker_item_list("Items", &state.ship_locker.items),
            locker_item_list("Components", &state.ship_locker.components),
            locker_item_list("Data", &state.ship_locker.data),
            locker_item_list("Consumables", &state.ship_locker.consumables),
        ]]
        .into()
    }
}

fn locker_item_list<'a>(title: &'a str, items: &'a [ShipLockerItem]) -> Column<'a, Message> {
    column![
        text(title).size(20).color(ORANGE),
        scrollable(column(
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
                                        .collect::<Vec<iced::Element<Message>>>()
                                ),
                                Position::FollowCursor,
                            )
                            .style(style::tooltip)
                        ]
                    }
                })
                .map(iced::Element::from)
        ))
        .style(style::scrollable)
        .width(Fill)
    ]
    .align_x(Left)
}
