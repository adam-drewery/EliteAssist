//! Provides a set of utility functions and macros for building UI components with the `iced` GUI library.
//!
//! # Functions
//!
//! - `sub_header`:
//!   Creates a `Row` containing a styled sub-header title.
//!   - `title`: Title text to be displayed.
//!   - Returns: A `Row` widget with the given title styled appropriately.
//!
//! - `details`:
//!   Displays a label-value pair in a horizontally aligned `Row`.
//!   - `label`: The text label for the data.
//!   - `value`: The value to display. If the value is empty, returns an empty row.
//!   - Returns: A `Row` with styled label and value.
//!
//! - `empty_placeholder`:
//!   Creates a placeholder
use crate::gui::Message;
use crate::theme::{style, GRAY, ORANGE};
use iced::widget::{column, row, scrollable, text, Column, Row, Scrollable};
use iced::{Element, Fill, Right};
use iced::padding::right;

/// Creates a sub-header component as a `Row` containing a single piece of styled text.
///
/// # Parameters
/// - `title`: A string slice (`&str`) that represents the text content for the sub-header.
///
/// # Returns
/// - A `Row<'_, Message>` containing the given title styled with the following properties:
///   - Font size: 20
///   - Color: `ORANGE`
///   - Width: `Fill` (fills the available space)
///
/// # Example
/// ```
/// let sub_header_row = sub_header("Settings");
/// ```
/// This will return a row displaying the text "Settings" styled with the above-mentioned properties.
pub fn sub_header(title: &str) -> Row<'_, Message> {
    row![text(title).size(20).color(ORANGE).width(Fill)]
}

///
/// A helper function to create a UI row displaying a labeled value pair.
/// This function generates a styled row where the label and value are visually distinct,
/// with the label aligned to the right and the value styled accordingly.
///
/// # Arguments
///
/// * `label` - A string slice (`&str`) that represents the label to be displayed.
/// * `value` - An input parameter implementing the `Into<String>` trait,
///             which represents the value to be displayed alongside the label.
///
/// # Returns
///
/// Returns a `Row<'_, Message>` that represents a structured UI row.
/// If the `value` is an empty string, an empty row is returned.
///
/// # Behavior
///
/// *
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

/// Creates a placeholder layout with a given label.
///
/// This function returns a `Column` component containing a rectangular placeholder
/// structure. The layout consists of three rows:
/// - The first and last rows are empty and span the available height using the `Fill` property.
/// - The middle row contains three columns:
///   - A left filler column spanning the available width using the `Fill` property.
///   - A center column containing the given `label` rendered as a `text` component.
///   - A right filler column also spanning the available width using the `Fill` property.
///
/// # Arguments
///
/// - `label`: A string slice (`&str`) representing the text that will be displayed
///   in the center column of the placeholder.
///
/// #
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

/// Creates a scrollable list container from a vector of elements.
///
/// This function takes a vector of elements that implement the `Into<Element<'a, Message>>` trait
/// and constructs a scrollable container displaying those elements in a vertical column.
/// The resulting scrollable container will have a right-padding and a custom style defined
/// by `style::scrollable`.
///
/// # Type Parameters
/// * `'a` - The lifetime of the elements and the scrollable container.
/// * `T` - A type that can be converted into an `Element` with a specific `Message`.
///
/// # Parameters
/// - `elements`: A vector of items implementing `Into<Element<'a, Message>>`, which
///   represents the content to be displayed in the scrollable list.
///
/// # Returns
/// A `Scrollable<'a, Message>` container displaying the given elements as a
pub fn scroll_list<'a, T: Into<Element<'a, Message>>>(elements: Vec<T>) -> Scrollable<'a, Message>
{
    let elements: Vec<Element<Message>> = elements.into_iter()
        .map(|e| e.into())
        .collect();

    scrollable(column(elements)
        .padding(right(12)))
        .width(Fill)
        .style(style::scrollable)
}

/// A macro to create a styled scrollable list using the `iced` UI framework.
///
/// # Parameters
/// - `$x:expr`:
///   A variadic parameter that accepts a list of expressions to be added as
///   elements to the scrollable list.
///
/// # Usage
/// ```rust
/// use my_crate::scroll_list;
/// use iced::{column, text};
///
/// let list = scroll_list![
///     text("Item 1"),
///     text("Item 2"),
///     text("Item 3")
/// ];
/// ```
///
/// # Generated Code
/// - Constructs a vertical column containing the provided elements.
/// - Adds right padding of 12 units to the column.
/// - Wraps the column in a scrollable container.
/// - Styles the scrollable container using the `style::scrollable` style.
///
/// # Returned Value
/// -
#[macro_export]
macro_rules! scroll_list {
    ($($x:expr),*) => {
        scrollable(
            column![$($x),*]
                .padding(iced::padding::right(12))
        )
        .width(iced::Fill)
        .style(style::scrollable)
    }
}

/// Macro `bordered_list_item` is designed to streamline the creation of bordered list items
/// with a specified layout and consistent styling.
///
/// # Parameters:
/// - `$($x:expr),*`: This is a variable number of expressions (items) that will be added to the row
///   inside the bordered container.
///
/// # Functionality:
/// - Constructs a container with a row that encapsulates the provided items (`$x`).
/// - Applies a pre-defined bordered style (`style::bordered`) to the container.
/// - Sets additional layout properties such as:
///   - `height`: Sets a fixed height of 64.
///   - `padding`: Adds padding of value `0.5` for the container.
///   - `width`: Sets the width of the container to fill the available space (`Fill`).
/// - Wraps the container in another row
#[macro_export]
macro_rules! bordered_list_item {
    ($($x:expr),*) => {
        row![
            container(
                row![$($x),*]
            )
            .style(style::bordered)
            .height(64)
            .padding(0.5)
            .width(Fill),
        ]
        .padding([4, 8])
    }
}

/// A macro to create a centered layout with Iced, a GUI library for Rust.
///
/// This macro helps layout components in a center-aligned configuration both vertically
/// and horizontally. It uses `iced`'s `row!` and `column!` macros to distribute empty
/// space around the provided widgets, ensuring that the content is centered within the
/// available layout space.
///
/// # Usage
///
/// The `centered!` macro takes a variadic list of widgets as its arguments. These widgets will
/// be positioned in the center of the layout.
///
/// ```rust
/// use iced::widget::{row, column};
/// use your_crate_name::centered;
///
/// // Center a single widget
/// let centered_content = centered![my_widget];
///
/// // Center multiple widgets
/// let centered_content = centered![widget_1, widget_2, widget_3];
/// ```
///
///
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

/// A macro to create a centered row layout in an `iced` UI.
/// The macro generates a row where the main content is horizontally centered
/// by adding two empty columns on either side with dynamic width (`iced::Fill`).
///
/// # Syntax
/// ```
/// centered_row!($($x:expr),*);
/// ```
///
/// - `$($x:expr),*`: A comma-separated list of expressions or widgets
/// that will be placed in the center column of the row.
///
/// # Example
/// ```rust
/// use iced::widget::{column, row};
/// use my_crate::centered_row;
///
/// let content = centered_row![
///     iced::widget::text("Centered"),
///     iced::widget::button("
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