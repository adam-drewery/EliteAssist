use crate::gui::{pane, Message};
use crate::theme::style;
use iced::widget::pane_grid;
use iced::widget::{column, row, text, Column, Row};
use iced::Fill;
use crate::gui::pane::modules::ship_modules;
use crate::gui::pane::navigation::{location, route};
use crate::gui::pane::personal::{claims, loadout, messages, missions, ranks};
use crate::gui::pane::ship::ship_details;
use crate::state::State;

pub fn overview(state: &State) -> Row<'_, Message> {
    if let Some(panes) = &state.layout.overview_panes {
        // Build a PaneGrid that contains all overview pane
        let grid = pane_grid::PaneGrid::new(panes, |_, kind, _| {
            let (title, content): (&str, Column<'_, Message>) = match kind {

                pane::Type::Loadout => ("Loadout", loadout(state)),
                pane::Type::Ranks => ("Ranks", ranks(state)),
                pane::Type::Messages => ("Messages", messages(state)),
                pane::Type::Route => ("Route", route(state)),
                pane::Type::Location => ("Location", location(state)),
                pane::Type::ShipDetails => ("Ship", ship_details(state)),
                pane::Type::ShipModules => ("Ship Modules", ship_modules(state)),
                pane::Type::Missions => ("Missions", missions(state)),
                pane::Type::Claims => ("Claims", claims(state)),
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