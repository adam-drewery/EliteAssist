use crate::gui::Message;
use crate::state::{Channel, State};
use crate::theme::{GRAY, ORANGE, WHITE, YELLOW};
use iced::widget::text::Wrapping;
use iced::widget::{column, row, scrollable, text, Column, Row};
use iced::{Color, Element, Fill, Left, Right};

pub fn messages(state: &State) -> Row<'_, Message> {
    row![
        messages_column(&state),
        journal_column(&state),
    ]
    .padding([0, 10])
}

pub fn name_color(kind: &Channel) -> Color {
    match kind {
        Channel::Local => GRAY,
        Channel::Squadron => ORANGE,
        Channel::SquadLeaders => YELLOW,
        _ => GRAY,
    }
}

fn messages_column(state: &State) -> Column<'_, Message> {
    column![
            text("Messages").size(20).color(ORANGE),
            scrollable(column(
                state
                    .messages
                    .iter()
                    .filter(|item| !item.from.is_empty())
                    .map(|item| {

                        row![
                            text(&item.from)
                                .size(16)
                                .color(name_color(&item.channel))
                                .height(16)
                                .align_x(Right)
                                .wrapping(Wrapping::None),
                            column![text(&item.text).color(WHITE).size(16)]
                                .padding([0, 8])
                                .width(Fill)
                                .align_x(Left),
                            column![
                                text(&item.time_display)
                                    .size(12)
                                    .color(GRAY)
                                    .width(192)
                                    .height(16)
                                    .align_x(Right)
                            ]
                            .padding([0, 8])
                        ]
                        .padding(2)
                    })
                    .map(Element::from)
            ))
            .anchor_bottom()
        ]
        .width(Fill)
        .align_x(Left)
}

fn journal_column(state: &State) -> Column<'_, Message> {
    column![
            text("Journal").size(20).color(ORANGE),
            scrollable(column(
                state
                    .logs
                    .iter()
                    .map(|item| {
                        row![
                            column![text(&item.verb).color(ORANGE).size(16)]
                                .padding([0, 4])
                                .width(192)
                                .align_x(Right),

                            text(&item.noun)
                                .size(16)
                                .color(WHITE)
                                .width(Fill)
                                .height(16)
                                .align_x(Left)
                                .wrapping(Wrapping::None),

                            column![text(&item.time_display)
                                .size(12)
                                    .color(GRAY)
                                    .width(192)
                                    .height(16)
                                    .align_x(Right)]
                                .padding([0, 8])
                        ]
                        .padding(2)
                    })
                    .map(Element::from)
            ))
            .anchor_bottom()
        ]
        .width(Fill)
        .align_x(Left)
}