use crate::font::EUROSTILE;
use crate::gui::components::scroll_list;
use crate::gui::{pane, Message};
use crate::state::State;
use crate::theme::{BLUE, GRAY, ORANGE, RED, WHITE, YELLOW};
use iced::widget::{column, row, text};
use iced::{Element, Fill};
use crate::state;

pub struct Messages;

impl pane::Type for Messages {
    fn title(&self) -> &'static str { "Messages" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        let cutoff = if state.layout.show_messages_days_limit > 0 {
            chrono::Utc::now().timestamp() - (state.layout.show_messages_days_limit as i64 * 24 * 60 * 60)
        } else {
            0
        };

        iced::widget::column![
            scroll_list(
                state
                    .messages
                    .iter()
                    .filter(|item| !item.from.is_empty() && item.timestamp >= cutoff)
                    .map(|item| {
                        let name_color = match item.channel {
                            state::chat::Channel::Local => ORANGE,
                            state::chat::Channel::Player => BLUE,
                            state::chat::Channel::StarSystem => ORANGE,
                            state::chat::Channel::Squadron => YELLOW,
                            state::chat::Channel::SquadLeaders => YELLOW,
                            state::chat::Channel::Npc => WHITE,
                            state::chat::Channel::Wing => BLUE,
                            state::chat::Channel::Unknown => RED,
                        };

                        column![
                            row![
                                column![text(item.from.as_ref()).size(16).color(name_color)],
                                column![].width(Fill),
                                column![text(item.time_display.as_ref()).size(12).color(GRAY)]
                                    .padding([0, 8]),
                            ],
                            row![
                                text(item.text.as_ref())
                                    .color(WHITE)
                                    .font(EUROSTILE)
                                    .size(16)
                            ]
                        ]
                        .width(Fill)
                    })
                    .collect()
            )
            .anchor_bottom()
        ]
        .into()
    }
}
