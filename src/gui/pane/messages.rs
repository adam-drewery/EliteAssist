use crate::font::EUROSTILE;
use crate::gui::Message;
use crate::state::{Channel, State};
use crate::theme::{GRAY, ORANGE, WHITE, YELLOW};
use iced::widget::text::Wrapping;
use iced::widget::{column, row, scrollable, text, Column};
use iced::{Color, Fill, Left, Right};

fn name_color(kind: &Channel) -> Color {
    match kind {
        Channel::Local => GRAY,
        Channel::Squadron => ORANGE,
        Channel::SquadLeaders => YELLOW,
        _ => GRAY,
    }
}

// Log (Messages) pane: replicates the left column of the old Messages screen
pub fn log_messages(state: &State) -> Column<'_, Message> {
    column![
        text("Messages").size(20).color(ORANGE),
        scrollable(column(
            state
                .messages
                .iter()
                .filter(|item| !item.from.is_empty())
                .map(|item| {
                    row![
                        text(item.from.as_ref())
                            .size(16)
                            .color(name_color(&item.channel))
                            .height(16)
                            .align_x(Right)
                            .wrapping(Wrapping::None),
                        column![
                            text(item.text.as_ref()).color(WHITE).font(EUROSTILE).size(16)
                        ]
                        .padding([0, 8])
                        .width(Fill)
                        .align_x(Left),
                        column![
                            text(item.time_display.as_ref())
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
                .map(iced::Element::from)
        ))
        .anchor_bottom()
    ]
    .width(Fill)
    .align_x(Left)
}
