use iced::widget::pane_grid;
use crate::gui::pane;
use crate::config;

#[derive(Default)]
pub struct Layout {
    pub overview_panes: Option<pane_grid::State<pane::Type>>,
    pub fullscreen: bool,
    pub custom_screens: Vec<config::CustomScreen>,
    pub selected_custom_screen: usize,
}

impl Layout {

    pub fn current_visible_vec(&self) -> Vec<pane::Type> {
        if self.custom_screens.is_empty() { return pane::Type::default_enabled_vec(); }
        let idx = self.selected_custom_screen.min(self.custom_screens.len().saturating_sub(1));
        if let Some(sel) = self.custom_screens.get(idx) {
            if let Some(v) = &sel.visible { return v.clone(); }
            if let Some(node) = &sel.layout { return config::layout_leaf_panes(node); }
        }
        pane::Type::default_enabled_vec()
    }

    pub fn set_current_visible_vec(&mut self, v: Vec<pane::Type>) {
        if self.custom_screens.is_empty() { return; }
        let idx = self.selected_custom_screen.min(self.custom_screens.len().saturating_sub(1));
        if let Some(sel) = self.custom_screens.get_mut(idx) {
            sel.visible = Some(v);
        }
    }

    pub fn sync_selected_custom_screen_from_live(&mut self) {
        // Ensure there is at least one custom screen entry
        if self.custom_screens.is_empty() { return; }
        let idx = self.selected_custom_screen.min(self.custom_screens.len().saturating_sub(1));
        if let Some(sel) = self.custom_screens.get_mut(idx) {
            // Update layout from current overview_panes
            if let Some(panes) = &self.overview_panes {
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
                        layout.overview_panes = Some(config::build_panes_from_layout(node));
                    }
                }
            } else {
                // Backward compatibility: single overview layout/visible
                if let Some(node) = &settings.layout {
                    layout.overview_panes = Some(config::build_panes_from_layout(node));
                }

                // If there were no custom screens, we'll create them below based on current state
            }
        }

        // If no panes were built from a saved layout, build default layout based on current visible set
        if layout.overview_panes.is_none() {
            pane::load(&mut layout);
        }

        // First run (no settings): create a default custom screen so it appears in the nav bar
        if layout.custom_screens.is_empty() {
            // Derive a layout node and visible set from the current live panes
            let (layout_node_opt, visible_opt) = if let Some(panes) = &layout.overview_panes {
                let node = config::state_to_node(panes);
                let visible = config::layout_leaf_panes(&node);
                (Some(node), Some(visible))
            } else {
                (None, Some(pane::Type::default_enabled_vec()))
            };

            layout.custom_screens.push(config::CustomScreen {
                name: "Overview".into(),
                layout: layout_node_opt,
                visible: visible_opt,
            });
            // Also add a default "Ship Locker" screen as a single pane
            layout.custom_screens.push(config::CustomScreen {
                name: "Materials".into(),
                layout: Some(config::LayoutNode::Pane(pane::Type::Materials)),
                visible: Some(vec![pane::Type::Materials]),
            });
            // Also add a default "Ship Locker" screen as a single pane
            layout.custom_screens.push(config::CustomScreen {
                name: "Ship Locker".into(),
                layout: Some(config::LayoutNode::Pane(pane::Type::ShipLocker)),
                visible: Some(vec![pane::Type::ShipLocker]),
            });
            // Also add a default "Market" screen as a single pane
            layout.custom_screens.push(config::CustomScreen {
                name: "Market".into(),
                layout: Some(config::LayoutNode::Pane(pane::Type::Market)),
                visible: Some(vec![pane::Type::Market]),
            });
            // Also add default Log panes as one combined screen
            layout.custom_screens.push(config::CustomScreen {
                name: "Logs".into(),
                layout: Some(config::LayoutNode::Split {
                    axis: config::AxisSer::Vertical,
                    ratio: 0.5,
                    a: Box::new(config::LayoutNode::Pane(pane::Type::Messages)),
                    b: Box::new(config::LayoutNode::Pane(pane::Type::LogJournal)),
                }),
                visible: Some(vec![pane::Type::Messages, pane::Type::LogJournal]),
            });
            layout.selected_custom_screen = 0;
        }

        layout
    }
}