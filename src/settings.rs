use std::fs;
use std::path::Path;

use iced::widget::pane_grid;
use serde::{Deserialize, Serialize};
use crate::state::pane;

/// File name for persisted settings
const SETTINGS_FILE: &str = "EliteAssist.config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub layout: Option<LayoutNode>,
    pub visible: Option<Vec<pane::Type>>, // explicit visible list; if None, derive from layout leaves
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
        let (layout, visible) = if let Some(panes) = &state.overview_panes {
            let layout = Some(to_layout_node(panes));
            let visible = state
                .enabled_panes
                .clone()
                .or_else(|| layout.as_ref().map(layout_leaf_panes));
            (layout, visible)
        } else {
            let visible = state
                .enabled_panes
                .clone()
                .or_else(|| Some(pane::Type::default_enabled_vec()));
            (None, visible)
        };

        let settings = Settings { layout, visible };
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
