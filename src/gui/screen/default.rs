use iced::widget::pane_grid;

use crate::config;
use crate::gui::pane;

/// Builds the default "Overview" layout as a serializable LayoutNode.
///
/// The structure mirrors the previous pane::defaults() layout by constructing
/// a temporary pane_grid::State and converting it to a LayoutNode.
pub fn overview_layout() -> config::LayoutNode {
    let (mut panes, pane_1) = pane_grid::State::new(pane::make("loadout"));

    if let Some((pane_2, split_1)) = panes.split(pane_grid::Axis::Vertical, pane_1, pane::make("route")) {
        if let Some((pane_3, _split_2)) = panes.split(pane_grid::Axis::Vertical, pane_2, pane::make("ship_details")) {
            if let Some((_, split_3)) = panes.split(pane_grid::Axis::Horizontal, pane_1, pane::make("messages")) {
                if let Some((_, split_4)) = panes.split(pane_grid::Axis::Horizontal, pane_1, pane::make("ranks")) {
                    if let Some((_, split_5)) = panes.split(pane_grid::Axis::Horizontal, pane_2, pane::make("location")) {
                        if let Some((_, split_6)) = panes.split(pane_grid::Axis::Horizontal, pane_3, pane::make("ship_modules")) {
                            // Set vertical split so each column is ~1/3 of the width
                            panes.resize(split_1, 1.0 / 3.0);
                            // Set horizontal splits within columns
                            panes.resize(split_3, 0.66);
                            panes.resize(split_4, 0.3);
                            panes.resize(split_5, 0.6);
                            panes.resize(split_6, 0.3);
                        }
                    }
                }
            }
        }
    }

    config::state_to_node(&panes)
}

/// Builds the full set of default custom screens shown on first run.
pub fn default_custom_screens() -> Vec<config::CustomScreen> {
    // Overview screen from the central layout
    let overview_node = overview_layout();
    let overview_visible = config::layout_leaf_panes(&overview_node);

    let mut screens = Vec::new();
    screens.push(config::CustomScreen {
        name: "Overview".into(),
        layout: Some(overview_node.clone()),
        visible: Some(overview_visible),
    });

    // Materials screen
    screens.push(config::CustomScreen {
        name: "Materials".into(),
        layout: Some(config::LayoutNode::Pane("materials".into())),
        visible: Some(vec!["materials".into()]),
    });

    // Ship Locker screen
    screens.push(config::CustomScreen {
        name: "Ship Locker".into(),
        layout: Some(config::LayoutNode::Pane("ship_locker".into())),
        visible: Some(vec!["ship_locker".into()]),
    });

    // Market screen
    screens.push(config::CustomScreen {
        name: "Market".into(),
        layout: Some(config::LayoutNode::Pane("market".into())),
        visible: Some(vec!["market".into()]),
    });

    // Logs screen: Messages over Journal
    screens.push(config::CustomScreen {
        name: "Logs".into(),
        layout: Some(config::LayoutNode::Split {
            axis: crate::config::AxisSer::Vertical,
            ratio: 0.5,
            a: Box::new(config::LayoutNode::Pane("messages".into())),
            b: Box::new(config::LayoutNode::Pane("log_journal".into())),
        }),
        visible: Some(vec!["messages".into(), "log_journal".into()]),
    });

    screens
}
