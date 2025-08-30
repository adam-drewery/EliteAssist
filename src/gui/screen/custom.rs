use crate::gui::{pane, Message};
use crate::state::State;
use crate::theme::style;
use iced::widget::{pane_grid, text, row, Column, Row};
use iced::Fill;

pub fn custom(state: &State) -> Row<'_, Message> {
    if let Some(panes) = &state.layout.current_panes {
        // Build a PaneGrid that contains all overview pane
        let grid = pane_grid::PaneGrid::new(panes, |_, kind, _| {
            let (title, content): (&str, Column<'_, Message>) = match kind {

                pane::Type::Loadout => ("Loadout", pane::loadout(state)),
                pane::Type::Ranks => ("Ranks", pane::ranks(state)),
                pane::Type::Messages => ("Messages", pane::messages(state)),
                pane::Type::Route => ("Route", pane::route(state)),
                pane::Type::Location => ("Location", pane::location(state)),
                pane::Type::ShipDetails => ("Ship", pane::ship_details(state)),
                pane::Type::ShipModules => ("Ship Modules", pane::ship_modules(state)),
                pane::Type::Missions => ("Missions", pane::missions(state)),
                pane::Type::Claims => ("Claims", pane::claims(state)),
                pane::Type::Materials => ("Materials", pane::materials(state)),
                pane::Type::ShipLocker => ("Ship Locker", pane::ship_locker(state)),
                pane::Type::Market => ("Market", pane::market(state)),
                pane::Type::LogJournal => ("Journal", pane::journal(state)),
            };

            pane_grid::Content::new(content.padding([8, 0]))
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
        row![]
    }
}