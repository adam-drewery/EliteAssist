use crate::font::EUROSTILE;
use crate::gui::components::scroll_list;
use crate::gui::{pane, Message};
use crate::state::State;
use crate::theme::{GRAY, ORANGE, WHITE};
use iced::widget::text::Wrapping;
use iced::widget::{column, row, text};
use iced::{Element, Fill, Left, Right};

pub struct LogJournal;

impl pane::Type for LogJournal {

    fn title(&self) -> &'static str { "Journal" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        column![
            scroll_list(
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
                    })
                .collect()
            )
            .anchor_bottom()
        ]
        .align_x(Left)
        .into()
    }
}
