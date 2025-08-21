mod navigation;
mod personal;
mod ship;
mod modules;


use crate::state::{State, PanelType};
use iced::widget::{column, row, Row};
use crate::gui::Message;
use iced::widget::pane_grid;
use iced::{Element, Fill};
use navigation::{location, route};
use modules::ship_modules;
use personal::{loadout, messages};
use ship::ship_details;

pub fn overview(state: &State) -> Row<'_, Message> {
    if let Some(panes) = &state.overview_panes {
        // Build a PaneGrid that contains all overview panel
        let grid = pane_grid::PaneGrid::new(panes, |_, kind, _| {
            let (title, content): (&str, Element<'_, Message>) = match kind {
                PanelType::Loadout => ("Loadout", loadout(state).into()),
                PanelType::Messages => ("Messages", messages(state).into()),
                PanelType::Route => ("Route", route(state).into()),
                PanelType::Location => ("Location", location(state).into()),
                PanelType::ShipDetails => ("Ship", ship_details(state).into()),
                PanelType::ShipModules => ("Modules", ship_modules(state).into()),
            };

            pane_grid::Content::new(content)
                .title_bar(pane_grid::TitleBar::new(title))
        })
        .width(Fill)
        .height(Fill)
        .on_drag(|e| Message::PaneDragged(e))
        .on_resize(10, |e| Message::PaneResized(e));

        row![grid].width(Fill)
    } else {
        // Fallback to the previous static layout until the panes are initialized
        row![
            column![
                loadout(state),
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