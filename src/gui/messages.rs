use crate::event::Event;
use crate::state::{MessageKind, State};
use crate::theme::{GRAY, ORANGE, WHITE};
use iced::widget::text::Wrapping;
use iced::widget::{column, row, scrollable, text, Column, Row};
use iced::{Color, Element, Fill, Left, Right};

pub fn messages(state: &State) -> Row<Event> {
    row![
        messages_column(&state),
        journal_column(&state),
    ]
    .padding([0, 10])
}

pub fn name_color(kind: &MessageKind) -> Color {
    match kind {
        MessageKind::Chat => ORANGE,
        MessageKind::System => GRAY,
        MessageKind::Ship => GRAY,
        MessageKind::Npc => GRAY,
    }
}

fn messages_column(state: &State) -> Column<Event> {
    column![
            text("MESSAGES").size(20).color(ORANGE),
            scrollable(column(
                state
                    .messages
                    .iter()
                    .filter(|item| !item.from.is_empty())
                    .map(|item| {

                        row![
                            text(&item.from)
                                .size(16)
                                .color(name_color(&item.kind))
                                .width(192)
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

fn join_location_parts(system: &str, body: &str, station: &Option<String>) -> String {
    let mut parts = Vec::new();
    if !system.is_empty() {
        parts.push(system);
    }
    if !body.is_empty() {
        parts.push(body);
    }
    if let Some(station) = station {
        if !station.is_empty() && !station.eq(body) {
            parts.push(station);
        }
    }
    parts.join(" | ")
}

fn journal_column(state: &State) -> Column<Event> {
    column![
            text("JOURNAL").size(20).color(ORANGE),
            scrollable(column(
                state
                    .journal
                    .iter()
                    .map(|item| {
                        let location = join_location_parts(&item.star_system, &item.body, &item.station);
                        row![
                            text(location)
                                .size(16)
                                .color(GRAY)
                                .width(384)
                                .height(16)
                                .align_x(Right)
                                .wrapping(Wrapping::None),
                            column![text(&item.text).color(WHITE).size(16)]
                                .padding([0, 8])
                                .width(Fill)
                                .align_x(Left),
                            text(&item.time_display)
                                .size(12)
                                    .color(GRAY)
                                    .width(192)
                                    .height(16)
                                    .align_x(Right)
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