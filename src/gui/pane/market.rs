use crate::font::EUROSTILE;
use crate::gui::components::sub_header;
use crate::gui::Message;
use crate::state::{MarketItem, State};
use crate::theme::{GRAY, ORANGE, WHITE, YELLOW};
use iced::widget::scrollable;
use iced::widget::{column, row, text, Column};
use iced::{Bottom, Element, Fill, Left};

// Market pane: table-like list of market groups/items inside a pane
pub fn market(state: &State) -> Column<'_, Message> {
    column![
        row![
            sub_header("Name"),
            sub_header("Buy"),
            sub_header("Sell"),
            sub_header("Stock"),
            sub_header("Demand"),
            sub_header("Producer"),
            sub_header("Consumer"),
        ]
        .spacing(20)
        .padding(2)
        .width(Fill),
        scrollable(column(state.market.groups.iter().flat_map(|group| {
            let mut rows: Vec<Element<Message>> = vec![
                text(group.name.as_ref())
                    .size(20)
                    .color(GRAY)
                    .align_x(Left)
                    .align_y(Bottom)
                    .height(32)
                    .into(),
            ];

            rows.extend(group.items.iter().map(|item| {
                row![
                    name_cell(item),
                    cell(item.buy_price),
                    cell(item.sell_price),
                    cell(item.stock),
                    cell(item.demand),
                    cell(item.producer),
                    cell(item.consumer),
                ]
                .spacing(20)
                .padding(2)
                .width(Fill)
                .into()
            }));

            rows
        })))
    ]
}

fn cell<'a>(value: impl text::IntoFragment<'a>) -> Element<'a, Message> {
    text(value).size(16).color(WHITE).width(Fill).into()
}

fn name_cell(item: &MarketItem) -> Element<'_, Message> {
    text(item.name.as_ref())
        .size(16)
        .color(if item.rare { YELLOW } else { ORANGE })
        .width(Fill)
        .font(EUROSTILE)
        .into()
}
