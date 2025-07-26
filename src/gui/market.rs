use crate::gui::components::header;
use crate::event::JournalEvent;
use crate::state::{MarketItem, State};
use crate::theme::{GRAY, ORANGE, WHITE, YELLOW};
use iced::widget::{column, row, text, Row};
use iced::widget::scrollable;
use iced::{Bottom, Element, Fill, Left};

pub fn market(state: &State) -> Row<JournalEvent> {
    row![
        column![
            row![
                header("NAME"),
                header("BUY"), 
                header("SELL"),
                header("STOCK"),
                header("DEMAND"), 
                header("PRODUCER"),
                header("CONSUMER"),
            ]
            .spacing(20)
            .padding(2)
            .width(Fill),
            scrollable(column(state.market.groups.iter().flat_map(|group| {
                let mut rows = vec![text(group.name.to_uppercase())
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

fn cell<'a>(value: impl text::IntoFragment<'a>) -> Element<'a, JournalEvent> {
    text(value).size(16).color(WHITE).width(Fill).into()
}

fn name_cell(item: &MarketItem) -> Element<JournalEvent> {
    text(&item.name)
        .size(16)
        .color(if item.rare { YELLOW } else { ORANGE })
        .width(Fill)
        .into()
}