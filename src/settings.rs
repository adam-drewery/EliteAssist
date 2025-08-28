use std::fs;
use std::path::Path;

use iced::widget::pane_grid;
use serde::{Deserialize, Serialize};
use crate::state::pane;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomScreen {
    pub name: Box<str>,
    pub layout: Option<LayoutNode>,
    pub visible: Option<Vec<pane::Type>>, // if None, derive from layout leaves
}

/// File name for persisted settings
const SETTINGS_FILE: &str = "EliteAssist.config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    // Back-compat fields for single-screen configuration
    pub layout: Option<LayoutNode>,
    pub visible: Option<Vec<pane::Type>>, // explicit visible list; if None, derive from layout leaves

    // New multi-screen configuration (optional for backward compatibility)
    pub custom_screens: Option<Vec<CustomScreen>>, // When present, overrides layout/visible
    pub selected_screen: Option<usize>,            // Index into custom_screens
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
    Pane(pane::Type),
}

impl Settings {
    pub fn save_from_state(state: &crate::state::State) -> std::io::Result<()> {
        // Determine current live layout and visible panes
        let (current_layout, current_visible) = if let Some(panes) = &state.overview_panes {
            (Some(to_layout_node(panes)), state.enabled_panes.clone().or_else(|| Some(layout_leaf_panes(&to_layout_node(panes)))))
        } else {
            (None, state.enabled_panes.clone().or_else(|| Some(pane::Type::default_enabled_vec())))
        };

        // Build multi-screen payload if available in state
        let mut custom_screens_opt = None;
        let mut selected_screen_opt = None;

        // If state tracks custom screens, persist them
        // Pull from state if available
        if !state.custom_screens.is_empty() {
            let mut screens_clone = state.custom_screens.clone();
            let selected_idx = state.selected_custom_screen.min(screens_clone.len().saturating_sub(1));
            if let Some(sel) = screens_clone.get_mut(selected_idx) {
                sel.layout = current_layout.clone();
                sel.visible = current_visible.clone();
            }
            custom_screens_opt = Some(screens_clone);
            selected_screen_opt = Some(selected_idx);
        }

        // For backward compatibility, mirror the selected screen into top-level fields
        let layout_bc = current_layout;
        let visible_bc = current_visible;

        let settings = Settings {
            layout: layout_bc,
            visible: visible_bc,
            custom_screens: custom_screens_opt,
            selected_screen: selected_screen_opt,
        };
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

pub fn to_layout_node(state: &pane_grid::State<pane::Type>) -> LayoutNode {
    // Traverse the Node tree and map panes to their PanelType via state.panes
    fn walk(node: &pane_grid::Node, state: &pane_grid::State<pane::Type>) -> LayoutNode {
        match node {
            pane_grid::Node::Split { axis, ratio, a, b, .. } => LayoutNode::Split {
                axis: axis.clone().into(),
                ratio: *ratio,
                a: Box::new(walk(a, state)),
                b: Box::new(walk(b, state)),
            },
            pane_grid::Node::Pane(p) => {
                if let Some(t) = state.panes.get(p) {
                    LayoutNode::Pane(t.clone())
                } else {
                    // Fallback: if missing, use a default panel
                    LayoutNode::Pane(pane::Type::Loadout)
                }
            }
        }
    }

    walk(state.layout(), state)
}

pub fn to_configuration(node: &LayoutNode) -> pane_grid::Configuration<pane::Type> {
    match node {
        LayoutNode::Pane(p) => pane_grid::Configuration::Pane(p.clone()),
        LayoutNode::Split { axis, ratio, a, b } => pane_grid::Configuration::Split {
            axis: axis.clone().into(),
            ratio: *ratio,
            a: Box::new(to_configuration(a)),
            b: Box::new(to_configuration(b)),
        },
    }
}

pub fn build_panes_from_layout(layout: &LayoutNode) -> pane_grid::State<pane::Type> {
    pane_grid::State::with_configuration(to_configuration(layout))
}

pub fn layout_leaf_panes(layout: &LayoutNode) -> Vec<pane::Type> {
    let mut v = Vec::new();
    fn walk(n: &LayoutNode, v: &mut Vec<pane::Type>) {
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
