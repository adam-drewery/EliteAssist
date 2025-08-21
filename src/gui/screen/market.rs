use crate::gui::components::header;
use crate::gui::Message;
use crate::state::{MarketItem, State};
use crate::theme::{GRAY, ORANGE, WHITE, YELLOW};
use iced::widget::scrollable;
use iced::widget::{column, row, text, Row};
use iced::{Bottom, Element, Fill, Left};
use crate::font::EUROSTILE;

pub fn market(state: &State) -> Row<'_, Message> {
    row![
        column![
            row![
                header("Name"),
                header("Buy"),
                header("Sell"),
                header("Stock"),
                header("Demand"),
                header("Producer"),
                header("Consumer"),
            ]
            .spacing(20)
            .padding(2)
            .width(Fill),
            scrollable(column(state.market.groups.iter().flat_map(|group| {
                let mut rows = vec![text(&group.name)
                    .size(20)
                    .color(GRAY)
                    .align_x(Left)
                    .align_y(Bottom)
                    .height(32)
                    .into()];
                
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
        .width(Fill)
    ]
}

fn cell<'a>(value: impl text::IntoFragment<'a>) -> Element<'a, Message> {
    text(value).size(16).color(WHITE).width(Fill).into()
}

fn name_cell(item: &MarketItem) -> Element<'_, Message> {
    text(&item.name)
        .size(16)
        .color(if item.rare { YELLOW } else { ORANGE })
        .width(Fill)
        .font(EUROSTILE)
        .into()
}