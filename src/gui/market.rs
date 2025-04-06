use crate::event::Event;
use crate::state::{MarketItem, State};
use crate::theme::{BLACK, ORANGE, WHITE, YELLOW};
use iced::widget::button::{Status, Style};
use iced::widget::{button, scrollable, Button};
use iced::widget::{column, row, text, Row};
use iced::Background::Color;
use iced::{Element, Fill, Theme};

pub fn market(state: &State) -> Row<Event> {
    row![
        column![
            row![
                header("NAME"),
                header("BUY"),
                header("SELL"),
                header("STOCK"),
                header("DEMAND"),
                header("CATEGORY"),
                header("PRODUCER"),
                header("CONSUMER"),
            ]
            .spacing(20)
            .padding(2)
            .width(Fill),
            scrollable(column(state.market.items.iter().map(|item| {
                row![
                    name_cell(&item),
                    cell(item.buy_price),
                    cell(item.sell_price),
                    cell(item.stock),
                    cell(item.demand),
                    cell(&item.category),
                    cell(item.producer),
                    cell(item.consumer),
                ]
                .spacing(20)
                .padding(2)
                .width(Fill)
                .into()
            })))
        ]
        .width(Fill)
    ]
}

fn header(name: &str) -> Button<Event> {
    button(name).width(Fill).style(column_title_style)
}

fn cell<'a>(value: impl text::IntoFragment<'a>) -> Element<'a, Event> {
    text(value).size(20).color(WHITE).width(Fill).into()
}

fn name_cell(item: &MarketItem) -> Element<Event> {
    text(&item.name)
        .size(20)
        .color(if item.rare { YELLOW } else { ORANGE })
        .width(Fill)
        .into()
}

fn column_title_style(_theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(Color(ORANGE)),
        text_color: BLACK,
        border: Default::default(),
        shadow: Default::default(),
    }
}
