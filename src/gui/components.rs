use crate::gui::Message;
use crate::theme::{style, GRAY, ORANGE};
use iced::widget::{column, row, scrollable, text, Column, Row, Scrollable};
use iced::{Element, Fill, Right};
use iced::padding::right;

pub fn sub_header(title: &str) -> Row<'_, Message> {
    row![text(title).size(20).color(ORANGE).width(Fill)]
}

pub fn details(label: &str, value: impl Into<String>) -> Row<'_, Message> {

    let value = value.into();
    if value == "" { return row![] } // todo: option

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

pub fn empty_placeholder(label: &str) -> Column<'_, Message> {

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

pub fn scroll_list<'a, T: Into<Element<'a, Message>>>(elements: Vec<T>) -> Scrollable<'a, Message>
{
    let elements: Vec<Element<Message>> = elements.into_iter()
        .map(|e| e.into())
        .collect();

    scrollable(column(elements)
        .padding(right(12)))
        .style(style::scrollable)
}

#[macro_export]
macro_rules! scroll_list {
    ($($x:expr),*) => {
        scrollable(
            column![$($x),*]
                .padding(iced::padding::right(12))
        )
        .style(style::scrollable)
    }
}

#[macro_export]
macro_rules! centered {
        ($($x:expr),*) => {
        column![
            row![].height(iced::Fill),
            row![
                column![].width(iced::Fill),
                column![$($x),*],
                column![].width(iced::Fill)
            ],
            row![].height(iced::Fill)
        ]
    }
}

#[macro_export]
macro_rules! centered_row {
    ($($x:expr),*) => {
        row![
            column![].width(iced::Fill),
            column![$($x),*],
            column![].width(iced::Fill),
        ]
    }
}