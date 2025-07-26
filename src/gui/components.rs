use crate::event::JournalEvent;
use crate::theme::styles::header_style;
use iced::widget::{container, row, column, text, Row};
use iced::{Fill, Right};
use crate::theme::{GRAY, ORANGE};

pub fn header(title: &str) -> Row<JournalEvent> {

    row![container(text(title).size(24).width(Fill)).style(header_style)].padding([12, 0])
}

pub fn sub_header(title: &str) -> Row<JournalEvent> {
    row![text(title).size(20).color(ORANGE)]
}

pub fn details(label: &str, value: String) -> Row<JournalEvent> {
    row![
            column![text(label).color(GRAY).size(20)]
                .align_x(Right)
                .padding([0, 8])
                .width(Fill),
            column![text(value).color(ORANGE).size(20)]
                .padding([0, 8])
                .width(Fill),
        ]
}