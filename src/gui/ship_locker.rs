use crate::font::eurocaps::FONT;
use crate::gui::Message;
use crate::material_locations::item_locations;
use crate::state::{ShipLockerItem, State};
use crate::theme::style;
use crate::theme::{BLUE, ORANGE, YELLOW};
use iced::widget::tooltip::Position;
use iced::widget::{column, row, scrollable, text, tooltip, Column, Row};
use iced::{Color, Element, Fill, Left, Top};

pub fn ship_locker(state: &State) -> Row<'_, Message> {

    row![
        locker_item_list("Items", &state.ship_locker.items),
        locker_item_list("Components", &state.ship_locker.components),
        locker_item_list("Data", &state.ship_locker.data),
        locker_item_list("Consumables", &state.ship_locker.consumables),
    ]
    .align_y(Top)
    .height(Fill)
}

pub fn locker_item_list<'a>(title: &'a str, items: &'a [ShipLockerItem]) -> Column<'a, Message> {

    column![
        text(title).font(FONT).size(20).color(ORANGE),
        scrollable(column(
            items
            .iter()
                .map(|item| {
                    let color = if item.for_mission { BLUE } else { Color::WHITE };

                    let content = row![
                        text(item.count).size(16).color(YELLOW).width(36),
                        text(&item.name).color(color).size(16)
                    ]
                    .padding(2);

                    // Determine locations for tooltip based on list title
                    let locations = item_locations(&item.name);

                    let tooltip_content: Option<Element<Message>> = locations.map(|locs| {
                        column(
                            locs
                                .iter()
                                .map(|loc| row![text(*loc).size(16)].into())
                                .collect::<Vec<Element<Message>>>()
                        )
                        .into()
                    });

                    match tooltip_content {
                        Some(content_col) => row![
                            tooltip(
                                content,
                                content_col,
                                Position::FollowCursor
                            )
                            .style(style::tooltip)
                        ],
                        None => row![content],
                    }
                })
                .map(Element::from)
        ))
        .width(Fill)
    ]
    .align_x(Left)
}