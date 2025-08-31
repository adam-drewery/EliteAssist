use crate::gui::Message;
use crate::state::State;
use crate::theme::style;
use iced::widget::{container, pane_grid, text, row, Row};
use iced::Fill;

pub fn custom(state: &State) -> Row<'_, Message> {
    if let Some(panes) = &state.layout.current_panes {
        // Build a PaneGrid that contains all overview pane
        let grid = pane_grid::PaneGrid::new(panes, |_, pane_obj, _| {
            let title = pane_obj.title();
            let content = pane_obj.render(state);

            pane_grid::Content::new(container(content).padding([8, 0]))
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