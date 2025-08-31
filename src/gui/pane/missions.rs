use crate::gui::components::*;
use crate::gui::Message;
use crate::state::State;
use crate::theme::style;
use iced::widget::{column, scrollable};
use iced::Element;
use thousands::Separable;

pub struct MissionsPane;

impl crate::gui::pane::PaneType for MissionsPane {

    fn title(&self) -> &'static str { "Missions" }
    
    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        
        if state.missions.len() == 0 {
            return column![empty_placeholder("No Missions")].into();
        }

        column![
            scrollable(column(
                state
                    .missions
                    .iter()
                    .map(|m| {
                        let mut c = column![
                            details(&m.faction, m.name.as_ref()),
                        ];

                        if let Some(commodity) = &m.commodity {
                            c = c.push(details("Commodity", commodity.as_ref()));
                        }
                        if let Some(count) = m.count {
                            c = c.push(details("Count", count.to_string()));
                        }
                        if let Some(dest) = &m.destination_system {
                            c = c.push(details("Destination System", dest.as_ref()));
                        }
                        if let Some(settlement) = &m.destination_settlement {
                            c = c.push(details("Destination Settlement", settlement.as_ref()));
                        }
                        if let Some(expiry) = m.expiry {
                            c = c.push(details("Expiry", expiry.to_rfc3339()));
                        }
                        c = c.push(details("Wing", if m.wing { "Yes" } else { "No" }));
                        c = c.push(details("Influence", m.influence.as_ref()));
                        c = c.push(details("Reputation", m.reputation.as_ref()));
                        if let Some(reward) = m.reward {
                            c = c.push(details("Reward", format!("CR {}", reward.separate_with_commas())));
                        }
                        c = c.push(details("Mission ID", m.mission_id.to_string()));

                        c
                    })
                    .map(Element::from)
            ))
            .style(style::scrollable)
        ]
        .into()
    }
}
