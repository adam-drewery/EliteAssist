use crate::font::EUROSTILE;
use crate::gui::{pane, Message};
use crate::state::{Channel, State};
use crate::theme::{style, GRAY, ORANGE, WHITE, YELLOW, BLUE};
use iced::widget::{column, row, scrollable, text};
use iced::{Element, Fill};

pub struct Messages;

impl pane::Type for Messages {

    fn title(&self) -> &'static str { "Messages" }
    
    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        iced::widget::column![
            scrollable(column(
                state
                    .messages
                    .iter()
                    .filter(|item| !item.from.is_empty())
                    .map(|item| {
                        let name_color = match item.channel {
                            Channel::Local => ORANGE,
                            Channel::Player => BLUE,
                            Channel::StarSystem => ORANGE,
                            Channel::Squadron => YELLOW,
                            Channel::SquadLeaders => YELLOW,
                            Channel::Npc => WHITE,
                            Channel::Wing => BLUE,
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
                    .map(Element::from)
            ))
            .style(style::scrollable)
            .anchor_bottom(),
            column![].width(12)
        ]
        .into()
    }
}
