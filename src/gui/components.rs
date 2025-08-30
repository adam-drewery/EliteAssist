use crate::gui::Message;
use crate::theme::style;
use crate::theme::{GRAY, ORANGE};
use iced::widget::{column, container, row, text, Column, Row};
use iced::{Fill, Right};

pub fn header(title: &str) -> Row<'_, Message> {
    row![
        container(text(title).size(24).width(Fill)).style(style::header).padding([0, 8])
    ]
    .padding([12, 0])
}

pub fn sub_header(title: &str) -> Row<'_, Message> {
    row![text(title).size(20).color(ORANGE).width(Fill)]
}

pub fn details(label: &str, value: impl Into<String>) -> Row<'_, Message> {

    let value = value.into();
    if value == "" { return row![] }

    row![
        column![text(label).color(GRAY).size(16)]
            .align_x(Right)
            .padding([0, 8])
            .width(Fill),
        column![text(value).color(ORANGE).size(16)]
            .padding([0, 8])
            .width(Fill),
    ]
}

pub fn empty_text(label: &str) -> Column<'_, Message> {
    column![
        row![].height(Fill),
        row![
            column![].width(Fill),
            column![text(label)],
            column![].width(Fill),
        ],
        row![].height(Fill),
    ]
}

#[macro_export]
macro_rules! centered_column {
    ($($x:expr),*) => {
        column![
            row![].height(Fill),
            row![$($x),*],
            row![].height(Fill),
        ]
    }
}

#[macro_export]
macro_rules! centered_row {
    ($($x:expr),*) => {
        row![
            column![].width(Fill),
            column![$($x),*],
            column![].width(Fill),
        ]
    }
}