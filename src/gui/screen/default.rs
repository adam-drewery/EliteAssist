use iced::widget::pane_grid;

use crate::config;
use crate::gui::pane;
use crate::gui::pane::*;

/// Builds the default "Overview" layout as a serializable LayoutNode.
pub fn overview_layout() -> config::LayoutNode {
    let (mut panes, pane_1) = pane_grid::State::<Box<dyn pane::Type>>::new(Box::new(Loadout));

    if let Some((pane_2, split_1)) = panes.split(pane_grid::Axis::Vertical, pane_1, Box::new(Route)) {
        if let Some((pane_3, _split_2)) = panes.split(pane_grid::Axis::Vertical, pane_2, Box::new(ShipDetails)) {
            if let Some((_, split_3)) = panes.split(pane_grid::Axis::Horizontal, pane_1, Box::new(Messages)) {
                if let Some((_, split_4)) = panes.split(pane_grid::Axis::Horizontal, pane_1, Box::new(Ranks)) {
                    if let Some((_, split_5)) = panes.split(pane_grid::Axis::Horizontal, pane_2, Box::new(Location)) {
                        if let Some((_, split_6)) = panes.split(pane_grid::Axis::Horizontal, pane_3, Box::new(ShipModules)) {
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

/// Builds the full set of default custom screens shown on the first run.
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
        layout: Some(config::LayoutNode::Pane(Materials.title().into())),
        visible: Some(vec![Materials.title().into()]),
    });

    // Ship Locker screen
    screens.push(config::CustomScreen {
        name: "Ship Locker".into(),
        layout: Some(config::LayoutNode::Pane(ShipLocker.title().into())),
        visible: Some(vec![ShipLocker.title().into()]),
    });

    // Market screen
    screens.push(config::CustomScreen {
        name: "Market".into(),
        layout: Some(config::LayoutNode::Pane(Market.title().into())),
        visible: Some(vec![Market.title().into()]),
    });

    // Logs screen: Messages over Journal
    screens.push(config::CustomScreen {
        name: "Logs".into(),
        layout: Some(config::LayoutNode::Split {
            axis: config::AxisSer::Vertical,
            ratio: 0.5,
            a: Box::new(config::LayoutNode::Pane(Messages.title().into())),
            b: Box::new(config::LayoutNode::Pane(LogJournal.title().into())),
        }),
        visible: Some(vec![Messages.title().into(), LogJournal.title().into()]),
    });

    screens
}
