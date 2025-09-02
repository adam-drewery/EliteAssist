use iced::widget::pane_grid;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::gui::pane;
use crate::state;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomScreen {
    pub name: Box<str>,
    pub layout: Option<LayoutNode>,
    pub visible: Option<Vec<Box<str>>>, // if None, derive from layout leaves
}

/// File name for persisted settings
const SETTINGS_FILE: &str = "EliteAssist.config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub layout: Option<LayoutNode>,
    pub visible: Option<Vec<Box<str>>>,
    pub custom_screens: Option<Vec<CustomScreen>>,
    pub selected_screen: Option<usize>,

    // Companion API auth
    pub capi_access_token: Option<Box<str>>,
    pub capi_refresh_token: Option<Box<str>>,
    pub capi_expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AxisSer {
    Horizontal,
    Vertical,
}

impl From<pane_grid::Axis> for AxisSer {
    fn from(a: pane_grid::Axis) -> Self {
        match a {
            pane_grid::Axis::Horizontal => AxisSer::Horizontal,
            pane_grid::Axis::Vertical => AxisSer::Vertical,
        }
    }
}

impl From<AxisSer> for pane_grid::Axis {
    fn from(a: AxisSer) -> Self {
        match a {
            AxisSer::Horizontal => pane_grid::Axis::Horizontal,
            AxisSer::Vertical => pane_grid::Axis::Vertical,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutNode {
    Split {
        axis: AxisSer,
        ratio: f32,
        a: Box<LayoutNode>,
        b: Box<LayoutNode>,
    },
    Pane(Box<str>),
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            layout: None,
            visible: None,
            custom_screens: None,
            selected_screen: None,
            capi_access_token: None,
            capi_refresh_token: None,
            capi_expires_at: None,
        }
    }
}

impl Settings {
    pub fn save_from_state(layout: &state::Layout) -> std::io::Result<()> {
        // Load existing to preserve CAPI tokens (and other fields we do not override here)
        let mut existing = Settings::load().unwrap_or_default();

        // Determine current live layout and visible panes
        let (current_layout, current_visible) = if let Some(panes) = &layout.current_panes {
            (Some(state_to_node(panes)), Some(layout.current_visible_vec()))
        } else {
            (None, Some(layout.current_visible_vec()))
        };

        // Build multi-screen payload if available in state
        let mut custom_screens_opt = None;
        let mut selected_screen_opt = None;

        // If state tracks custom screens, persist them
        if !layout.custom_screens.is_empty() {
            let mut screens_clone = layout.custom_screens.clone();
            let selected_idx = layout.selected_custom_screen.min(screens_clone.len().saturating_sub(1));
            if let Some(sel) = screens_clone.get_mut(selected_idx) {
                sel.layout = current_layout.clone();
                if let Some(v) = &current_visible {
                    sel.visible = Some(v.iter().map(|p| p.as_ref().title().into()).collect());
                }
            }
            custom_screens_opt = Some(screens_clone);
            selected_screen_opt = Some(selected_idx);
        }

        // Mirror the selected screen into top-level fields for compatibility
        existing.layout = current_layout;
        existing.visible = current_visible.map(|v| v.iter().map(|p| p.as_ref().title().into()).collect());
        existing.custom_screens = custom_screens_opt;
        existing.selected_screen = selected_screen_opt;

        let json = serde_json::to_string_pretty(&existing).unwrap_or_else(|_| "{}".into());
        fs::write(SETTINGS_FILE, json)
    }

    pub fn save_tokens(access_token: &str, refresh_token: &str, expires_at: i64) -> std::io::Result<()> {
        let mut settings = Settings::load().unwrap_or_default();
        settings.capi_access_token = Some(access_token.into());
        settings.capi_refresh_token = Some(refresh_token.into());
        settings.capi_expires_at = Some(expires_at);
        let json = serde_json::to_string_pretty(&settings).unwrap_or_else(|_| "{}".into());
        fs::write(SETTINGS_FILE, json)
    }

    pub fn load() -> Option<Settings> {
        if !Path::new(SETTINGS_FILE).exists() {
            return None;
        }
        let data = fs::read_to_string(SETTINGS_FILE).ok()?;
        serde_json::from_str(&data).ok()
    }
}

pub fn to_configuration(node: &LayoutNode) -> pane_grid::Configuration<Box<dyn pane::Type>> {
    match node {
        LayoutNode::Pane(id) => pane_grid::Configuration::Pane(pane::from_title(id.as_ref())),
        LayoutNode::Split { axis, ratio, a, b } => pane_grid::Configuration::Split {
            axis: axis.clone().into(),
            ratio: *ratio,
            a: Box::new(to_configuration(a)),
            b: Box::new(to_configuration(b)),
        },
    }
}

pub fn build_panes_from_layout(layout: &LayoutNode) -> pane_grid::State<Box<dyn pane::Type>> {
    pane_grid::State::with_configuration(to_configuration(layout))
}

pub fn layout_leaf_panes(layout: &LayoutNode) -> Vec<Box<str>> {
    let mut v: Vec<Box<str>> = Vec::new();
    fn walk(n: &LayoutNode, v: &mut Vec<Box<str>>) {
        match n {
            LayoutNode::Pane(p) => v.push(p.clone()),
            LayoutNode::Split { a, b, .. } => {
                walk(a, v);
                walk(b, v);
            }
        }
    }
    walk(layout, &mut v);
    v
}

pub fn state_to_node(state: &pane_grid::State<Box<dyn pane::Type>>) -> LayoutNode {
    // Traverse the Node tree and map panes to their IDs via state.panes
    fn walk(node: &pane_grid::Node, state: &pane_grid::State<Box<dyn pane::Type>>) -> LayoutNode {
        match node {
            pane_grid::Node::Split { axis, ratio, a, b, .. } => LayoutNode::Split {
                axis: axis.clone().into(),
                ratio: *ratio,
                a: Box::new(walk(a, state)),
                b: Box::new(walk(b, state)),
            },
            pane_grid::Node::Pane(p) => {
                if let Some(t) = state.panes.get(p) {
                    LayoutNode::Pane(t.title().into())
                } else {
                    // Fallback: if missing, use a default pane
                    LayoutNode::Pane("Loadout".into())
                }
            }
        }
    }

    walk(state.layout(), state)
}