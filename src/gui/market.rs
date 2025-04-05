use crate::event::Event;
use crate::state::State;
use crate::theme::{BLACK, GRAY, ORANGE, WHITE, YELLOW};
use iced::widget::{button, scrollable};
use iced::widget::{column, row, text, Row};
use iced::{Fill, Theme};
use iced::Background::Color;
use iced::widget::button::{Status, Style};

pub fn market(state: &State) -> Row<Event> {
    row![
        column![
            row![
                button("NAME").width(Fill).style(column_title_style),
                button("BUY").width(Fill).style(column_title_style),
                button("SELL").width(Fill).style(column_title_style),
                button("STOCK").width(Fill).style(column_title_style),
                button("DEMAND").width(Fill).style(column_title_style),
                button("CATEGORY").width(Fill).style(column_title_style),
                button("PRODUCER").width(Fill).style(column_title_style),
                button("CONSUMER").width(Fill).style(column_title_style),
            ]
            .spacing(20)
            .padding(2)
            .width(Fill),
            scrollable(column(state.market.items.iter().map(|item| {
                row![
                    text(&item.name)
                        .size(20)
                        .color(if item.rare { YELLOW } else { ORANGE })
                        .width(Fill),
                    text(item.buy_price).size(20).color(WHITE).width(Fill),
                    text(item.sell_price).size(20).color(WHITE).width(Fill),
                    text(item.stock).size(20).color(GRAY).width(Fill),
                    text(item.demand).size(20).color(WHITE).width(Fill),
                    text(&item.category).size(20).color(GRAY).width(Fill),
                    text(item.producer).size(20).color(GRAY).width(Fill),
                    text(item.consumer).size(20).color(GRAY).width(Fill),
                    
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

fn column_title_style(_theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(Color(ORANGE)),
        text_color: BLACK,
        border: Default::default(),
        shadow: Default::default(),
    }
}