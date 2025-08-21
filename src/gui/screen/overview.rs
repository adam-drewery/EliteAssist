use crate::gui::Message;
use crate::state::{PanelType, State};
use crate::theme::style;
use iced::widget::pane_grid;
use iced::widget::{column, row, text, Column, Row};
use iced::Fill;
use crate::gui::panel::modules::ship_modules;
use crate::gui::panel::navigation::{location, route};
use crate::gui::panel::personal::{claims, loadout, messages, missions, ranks};
use crate::gui::panel::ship::ship_details;

pub fn overview(state: &State) -> Row<'_, Message> {
    if let Some(panes) = &state.overview_panes {
        // Build a PaneGrid that contains all overview panel
        let grid = pane_grid::PaneGrid::new(panes, |_, kind, _| {
            let (title, content): (&str, Column<'_, Message>) = match kind {

                PanelType::Loadout => ("Loadout", loadout(state)),
                PanelType::Ranks => ("Ranks", ranks(state)),
                PanelType::Messages => ("Messages", messages(state)),
                PanelType::Route => ("Route", route(state)),
                PanelType::Location => ("Location", location(state)),
                PanelType::ShipDetails => ("Ship", ship_details(state)),
                PanelType::ShipModules => ("Ship Modules", ship_modules(state)),
                PanelType::Missions => ("Missions", missions(state)),
                PanelType::Claims => ("Claims", claims(state)),
            };

            pane_grid::Content::new(content)
                .title_bar(
                    pane_grid::TitleBar::new(
                        text(title).size(24)
                    )
                    .style(style::header)
                    .padding([0, 8])
                )
        })
        .width(Fill)
        .height(Fill)
        .spacing(8)
        .on_drag(|e| Message::PaneDragged(e))
        .on_resize(10, |e| Message::PaneResized(e));

        row![grid].width(Fill)
    } else {
        // Fallback to the previous static layout until the panes are initialized
        row![
            column![
                loadout(state),
                ranks(state),
                messages(state)
            ],
            column![
                route(state),
                location(state)
            ],
            column![
                ship_details(state),
                ship_modules(state)
            ]
        ]
    }
}