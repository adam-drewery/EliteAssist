use iced::widget::pane_grid;
use crate::gui::pane;
use crate::config;
use crate::gui::pane::Type;

#[derive(Default)]
pub struct Layout {
    pub fullscreen: bool,
    pub custom_screens: Vec<config::CustomScreen>,
    pub current_panes: Option<pane_grid::State<Box<dyn Type>>>,
    pub selected_custom_screen: usize,
}

impl Layout {

    pub fn current_visible_vec(&self) -> Vec<Box<dyn Type>> {
        if self.custom_screens.is_empty() {
            return pane::defaults()
                .into_iter()
                .map(|s| pane::from_title(s.title()))
                .collect();
        }
        let idx = self.selected_custom_screen.min(self.custom_screens.len().saturating_sub(1));
        if let Some(sel) = self.custom_screens.get(idx) {
            if let Some(v) = &sel.visible {
                return v.iter().map(|t| pane::from_title(t.as_ref())).collect();
            }
            if let Some(node) = &sel.layout {
                let titles = config::layout_leaf_panes(node);
                return titles.iter().map(|t| pane::from_title(t.as_ref())).collect();
            }
        }
        pane::defaults()
            .into_iter()
            .map(|s| pane::from_title(s.title()))
            .collect()
    }

    pub fn set_current_visible_vec(&mut self, v: Vec<Box<dyn Type>>) {
        if self.custom_screens.is_empty() { return; }
        let idx = self.selected_custom_screen.min(self.custom_screens.len().saturating_sub(1));
        if let Some(sel) = self.custom_screens.get_mut(idx) {
            sel.visible = Some(v.iter().map(|p| p.as_ref().title().into()).collect());
        }
    }

    pub fn sync_selected_custom_screen_from_live(&mut self) {
        // Ensure there is at least one custom screen entry
        if self.custom_screens.is_empty() { return; }
        let idx = self.selected_custom_screen.min(self.custom_screens.len().saturating_sub(1));
        if let Some(sel) = self.custom_screens.get_mut(idx) {
            // Update layout from current overview_panes
            if let Some(panes) = &self.current_panes {
                let layout = config::state_to_node(panes);
                sel.layout = Some(layout.clone());
                // Derive visible panes from layout leaves
                sel.visible = Some(config::layout_leaf_panes(&layout));
            }
        }
    }

    pub fn from_settings() -> Layout {
        let mut layout = Layout::default();

        // Attempt to load persisted settings and apply
        if let Some(settings) = config::Settings::load() {
            if let Some(screens) = settings.custom_screens.clone() {
                // Use multi-screen config
                layout.custom_screens = screens.clone();
                let len = layout.custom_screens.len();
                layout.selected_custom_screen = settings.selected_screen.unwrap_or(0).min(len.saturating_sub(1).max(0));

                if let Some(sel) = layout.custom_screens.get(layout.selected_custom_screen) {
                    if let Some(node) = &sel.layout {
                        layout.current_panes = Some(config::build_panes_from_layout(node));
                    }
                }
            } else {
                // Backward compatibility: single overview layout/visible
                if let Some(node) = &settings.layout {
                    layout.current_panes = Some(config::build_panes_from_layout(node));
                }
            }
        }

        // If no panes were built from a saved layout, build default layout using screen defaults
        if layout.current_panes.is_none() {
            let node = crate::gui::screen::default::overview_layout();
            layout.current_panes = Some(config::build_panes_from_layout(&node));
        }

        // First run (no settings): populate default custom screens via central defaults
        if layout.custom_screens.is_empty() {
            layout.custom_screens = crate::gui::screen::default::default_custom_screens();
            layout.selected_custom_screen = 0;

            // Initialize overview panes from the selected screen's layout
            if let Some(sel) = layout.custom_screens.get(layout.selected_custom_screen) {
                if let Some(node) = &sel.layout {
                    layout.current_panes = Some(config::build_panes_from_layout(node));
                }
            }
        }

        layout
    }
}