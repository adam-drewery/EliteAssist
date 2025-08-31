use iced::widget::pane_grid;

use crate::config;
use crate::gui::pane::{LoadoutPane, LocationPane, LogJournalPane, MarketPane, MaterialsPane, MessagesPane, PaneType, RanksPane, RoutePane, ShipDetailsPane, ShipLockerPane, ShipModulesPane};

/// Builds the default "Overview" layout as a serializable LayoutNode.
pub fn overview_layout() -> config::LayoutNode {
    let (mut panes, pane_1) = pane_grid::State::<Box<dyn PaneType>>::new(Box::new(LoadoutPane));

    if let Some((pane_2, split_1)) = panes.split(pane_grid::Axis::Vertical, pane_1, Box::new(RoutePane)) {
        if let Some((pane_3, _split_2)) = panes.split(pane_grid::Axis::Vertical, pane_2, Box::new(ShipDetailsPane)) {
            if let Some((_, split_3)) = panes.split(pane_grid::Axis::Horizontal, pane_1, Box::new(MessagesPane)) {
                if let Some((_, split_4)) = panes.split(pane_grid::Axis::Horizontal, pane_1, Box::new(RanksPane)) {
                    if let Some((_, split_5)) = panes.split(pane_grid::Axis::Horizontal, pane_2, Box::new(LocationPane)) {
                        if let Some((_, split_6)) = panes.split(pane_grid::Axis::Horizontal, pane_3, Box::new(ShipModulesPane)) {
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
        layout: Some(config::LayoutNode::Pane(MaterialsPane.title().into())),
        visible: Some(vec![MaterialsPane.title().into()]),
    });

    // Ship Locker screen
    screens.push(config::CustomScreen {
        name: "Ship Locker".into(),
        layout: Some(config::LayoutNode::Pane(ShipLockerPane.title().into())),
        visible: Some(vec![ShipLockerPane.title().into()]),
    });

    // Market screen
    screens.push(config::CustomScreen {
        name: "Market".into(),
        layout: Some(config::LayoutNode::Pane(MarketPane.title().into())),
        visible: Some(vec![MarketPane.title().into()]),
    });

    // Logs screen: Messages over Journal
    screens.push(config::CustomScreen {
        name: "Logs".into(),
        layout: Some(config::LayoutNode::Split {
            axis: config::AxisSer::Vertical,
            ratio: 0.5,
            a: Box::new(config::LayoutNode::Pane(MessagesPane.title().into())),
            b: Box::new(config::LayoutNode::Pane(LogJournalPane.title().into())),
        }),
        visible: Some(vec![MessagesPane.title().into(), LogJournalPane.title().into()]),
    });

    screens
}
