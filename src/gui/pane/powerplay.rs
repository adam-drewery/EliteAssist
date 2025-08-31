use iced::widget::{column, scrollable};
use iced::Element;

use crate::gui::components::{details, empty_placeholder};
use crate::gui::{pane, Message};
use crate::state::State;
use crate::theme::style;
use thousands::Separable;

pub struct Powerplay;

impl pane::Type for Powerplay {

    fn title(&self) -> &'static str { "Powerplay" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        if state.powerplay.power.is_none() {
            return column![
                scrollable(
                    column![
                        empty_placeholder("Not pledged to any Power")
                    ]
                ).style(style::scrollable)
            ].into();
        }

        let pp = &state.powerplay;

        let merits_str = pp.merits.separate_with_commas();
        let time_pledged_str = if pp.time_pledged == 0 {
            String::new()
        } else {
            // Simple humanized days:hours for readability
            let secs = pp.time_pledged;
            let days = secs / 86_400;
            let hours = (secs % 86_400) / 3_600;
            if days > 0 { format!("{}d {}h", days, hours) } else { format!("{}h", hours) }
        };

        column![
            scrollable(
                column![
                    details("Pledged Power", pp.power.clone().unwrap_or_default()),
                    details(
                        "Rank",
                        pp.rank.map(|r| r.to_string()).unwrap_or_else(|| "Unknown".to_string())
                    ),
                    details("Merits", merits_str),
                    details("Time Pledged", time_pledged_str),
                    details(
                        "Last Salary",
                        pp.last_salary
                            .map(|a| format!("{} CR", a.separate_with_commas()))
                            .unwrap_or_default()
                    )
                ]
            ).style(style::scrollable)
        ]
        .into()
    }
}
