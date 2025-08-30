use crate::font::EUROSTILE;
use crate::gui::Message;
use crate::state::State;
use crate::theme::{style, GRAY, ORANGE, WHITE};
use iced::widget::text::Wrapping;
use iced::widget::{column, row, scrollable, text, Column};
use iced::{Fill, Left, Right};

pub fn journal(state: &State) -> Column<'_, Message> {
    column![
        scrollable(
            column(
                state
                    .logs
                    .iter()
                    .map(|item| {
                        row![
                            column![
                                text(item.verb.as_ref())
                                    .color(ORANGE)
                                    .size(16)
                                    .font(EUROSTILE)
                            ]
                            .padding([0, 4])
                            .width(192)
                            .align_x(Right),
    
                            text(item.noun.as_ref())
                                .size(16)
                                .color(WHITE)
                                .width(Fill)
                                .height(16)
                                .align_x(Left)
                                .wrapping(Wrapping::None)
                                .font(EUROSTILE),
    
                            column![].width(Fill),
                        
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
                    }
                ).map(iced::Element::from)
            )
        )
        .anchor_bottom()
        .style(style::scrollable)
    ]
    .align_x(Left)
}
