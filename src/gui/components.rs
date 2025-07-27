use crate::event::JournalEvent;
use crate::theme::styles::header_style;
use crate::theme::{GRAY, ORANGE};
use iced::widget::{column, container, row, text, Row};
use iced::{Fill, Right};
use crate::fonts::eurocaps::FONT;

pub fn header(title: &str) -> Row<JournalEvent> {
    row![
        container(text(title).font(FONT).size(24).width(Fill)).style(header_style)
    ]
    .padding([12, 0])
}

pub fn sub_header(title: &str) -> Row<JournalEvent> {
    row![text(title).font(FONT).size(20).color(ORANGE)]
}

pub fn details(label: &str, value: impl Into<String>) -> Row<JournalEvent> {
    
    let value = value.into();
    if value == "" { return row![] }
    
    row![
        column![text(label).font(FONT).color(GRAY).size(20)]
            .align_x(Right)
            .padding([0, 8])
            .width(Fill),
        column![text(value).font(FONT).color(ORANGE).size(20)]
            .padding([0, 8])
            .width(Fill),
    ]
}