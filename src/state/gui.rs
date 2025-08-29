use iced::widget::pane_grid;
use crate::gui::pane;

impl Layout {
    pub fn sync_selected_custom_screen_from_live(&mut self) {
        // Ensure there is at least one custom screen entry
        if self.custom_screens.is_empty() { return; }
        let idx = self.selected_custom_screen.min(self.custom_screens.len().saturating_sub(1));
        if let Some(sel) = self.custom_screens.get_mut(idx) {
            // Update layout from current overview_panes
            if let Some(panes) = &self.overview_panes {
                let layout = crate::settings::state_to_node(panes);
                sel.layout = Some(layout);
            }
            // Update visible panes list from current state
            let visible = self.enabled_panes.clone().unwrap_or_else(|| pane::Type::default_enabled_vec());
            sel.visible = Some(visible);
        }
    }
}

#[derive(Default)]
pub struct Layout {
    pub overview_panes: Option<pane_grid::State<pane::Type>>,
    pub fullscreen: bool,
    pub enabled_panes: Option<Vec<pane::Type>>,
    pub custom_screens: Vec<crate::settings::CustomScreen>,
    pub selected_custom_screen: usize,
}

impl Layout {
    pub fn from_settings() -> Layout {
        let mut layout = Layout::default();

        // Attempt to load persisted settings and apply
        if let Some(settings) = crate::settings::Settings::load() {
            if let Some(screens) = settings.custom_screens.clone() {
                // Use multi-screen config
                layout.custom_screens = screens.clone();
                let len = layout.custom_screens.len();
                layout.selected_custom_screen = settings.selected_screen.unwrap_or(0).min(len.saturating_sub(1).max(0));

                if let Some(sel) = layout.custom_screens.get(layout.selected_custom_screen) {
                    if let Some(node) = &sel.layout {
                        layout.overview_panes = Some(crate::settings::build_panes_from_layout(node));
                        layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| crate::settings::layout_leaf_panes(node)));
                    } else {
                        layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| pane::Type::default_enabled_vec()));
                    }
                }
            } else {
                // Backward compatibility: single overview layout/visible
                if let Some(node) = &settings.layout {
                    layout.overview_panes = Some(crate::settings::build_panes_from_layout(node));
                    layout.enabled_panes = Some(settings.visible.clone().unwrap_or_else(|| crate::settings::layout_leaf_panes(node)));
                } else if let Some(visible) = settings.visible.clone() {
                    layout.enabled_panes = Some(visible);
                }

                // Initialize custom_screens with a single entry named "Overview"
                layout.custom_screens.push(crate::settings::CustomScreen {
                    name: "Overview".into(),
                    layout: settings.layout.clone(),
                    visible: settings.visible.clone(),
                });
                layout.selected_custom_screen = 0;
            }
        }

        // If no panes were built from a saved layout, build default layout based on enabled set
        if layout.overview_panes.is_none() {
            pane::load(&mut layout);
        }

        // First run (no settings): create a default custom screen so it appears in the nav bar
        if layout.custom_screens.is_empty() {
            // Derive a layout node and visible set from the current live panes
            let (layout_node_opt, visible_opt) = if let Some(panes) = &layout.overview_panes {
                let node = crate::settings::state_to_node(panes);
                let visible = layout
                    .enabled_panes
                    .clone()
                    .unwrap_or_else(|| crate::settings::layout_leaf_panes(&node));
                (Some(node), Some(visible))
            } else {
                (None, Some(pane::Type::default_enabled_vec()))
            };

            layout.custom_screens.push(crate::settings::CustomScreen {
                name: "Overview".into(),
                layout: layout_node_opt,
                visible: visible_opt,
            });
            layout.selected_custom_screen = 0;
        }

        layout
    }
}