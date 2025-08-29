mod market;
mod materials;
mod messages;
mod overview;
mod ship_locker;
mod settings;

use log::error;
pub use market::*;
pub use materials::*;
pub use messages::*;
pub use overview::*;
pub use ship_locker::*;
pub use settings::*;
use crate::gui::pane;
use crate::state::{Layout, Screen};

pub fn add_custom(layout: &mut Layout) {
    
    // Clone the current layout/visibility into a new screen
    let (layout_opt, visible_opt) = if let Some(panes) = &layout.overview_panes {
        let layout_node = crate::settings::state_to_node(panes);
        let visible = layout.enabled_panes
            .clone()
            .unwrap_or_else(|| crate::settings::layout_leaf_panes(&layout_node));
        (Some(layout_node), Some(visible))
    } else {
        (None, Some(layout.enabled_panes.clone().unwrap_or_else(|| pane::Type::default_enabled_vec())))
    };

    let name = format!("Screen {}", layout.custom_screens.len() + 1).into();
    layout.custom_screens.push(crate::settings::CustomScreen {
        name,
        layout: layout_opt,
        visible: visible_opt,
    });
    layout.selected_custom_screen = layout.custom_screens.len().saturating_sub(1);
    crate::settings::Settings::save_from_state(&layout)
        .unwrap_or_else(|_| error!("Failed to save layout"));
}

pub fn remove_custom(layout: &mut Layout) {

    if layout.custom_screens.len() > 1 {
        let idx = layout.selected_custom_screen.min(layout.custom_screens.len() - 1);
        layout.custom_screens.remove(idx);

        // Clamp selection
        if layout.selected_custom_screen >= layout.custom_screens.len() {
            if layout.custom_screens.is_empty() { layout.selected_custom_screen = 0; } else { layout.selected_custom_screen = layout.custom_screens.len() - 1; }
        }

        // Load the selected screen configuration
        if let Some(sel) = layout.custom_screens.get(layout.selected_custom_screen) {
            if let Some(node) = &sel.layout {
                layout.overview_panes = Some(crate::settings::build_panes_from_layout(node));
                layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| crate::settings::layout_leaf_panes(node)));
            } else {
                layout.overview_panes = None;
                layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| pane::Type::default_enabled_vec()));
            }
        }
        
        crate::settings::Settings::save_from_state(&layout)
            .unwrap_or_else(|_| error!("Failed to save layout"));
    }
}

pub fn select_custom(layout: &mut Layout, idx: usize) {
    if !layout.custom_screens.is_empty() {
        layout.selected_custom_screen = idx.min(layout.custom_screens.len() - 1);
        if let Some(sel) = layout.custom_screens.get(layout.selected_custom_screen) {
            if let Some(node) = &sel.layout {
                layout.overview_panes = Some(crate::settings::build_panes_from_layout(node));
                layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| crate::settings::layout_leaf_panes(node)));
            } else {
                layout.overview_panes = None;
                layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| pane::Type::default_enabled_vec()));
            }
        }
        crate::settings::Settings::save_from_state(&layout)
            .unwrap_or_else(|_| error!("Failed to save layout"));
    }
}

pub fn rename_custom(layout: &mut Layout, name: Box<str>) {
    if let Some(sel) = layout.custom_screens.get_mut(layout.selected_custom_screen) {
        sel.name = name;
        crate::settings::Settings::save_from_state(&layout)
            .unwrap_or_else(|_| error!("Failed to save layout"));
    }
}

pub fn navigate_to(layout: &mut Layout, idx: usize) -> Screen {
    if !layout.custom_screens.is_empty() {
        let max_idx = layout.custom_screens.len().saturating_sub(1);
        layout.selected_custom_screen = idx.min(max_idx);
        if let Some(sel) = layout.custom_screens.get(layout.selected_custom_screen) {
            if let Some(node) = &sel.layout {
                layout.overview_panes = Some(crate::settings::build_panes_from_layout(node));
                layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| crate::settings::layout_leaf_panes(node)));
            } else {
                layout.overview_panes = None;
                layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| pane::Type::default_enabled_vec()));
            }
        }
        // Switch to Commander screen which displays the selected custom screen layout
        crate::settings::Settings::save_from_state(&layout)
            .unwrap_or_else(|_| error!("Failed to save layout"));
    }

    Screen::Commander
}

pub(crate) fn next_tab(layout: &mut Layout, active_screen: &Screen) -> Option<Screen> {
    // Determine total tabs in navigation bar: custom screens + Materials + Ship Locker + Market + Log
    let custom_count = layout.custom_screens.len();
    let fixed_count = 4usize; // Materials, Ship Locker, Market, Messages(Log)
    let total = custom_count + fixed_count;
    if total == 0 { return None; }

    // Map current state to index
    let current_index = match active_screen {
        Screen::Commander => {
            if custom_count == 0 { custom_count } else { layout.selected_custom_screen.min(custom_count.saturating_sub(1)) }
        }
        Screen::Materials => custom_count,
        Screen::ShipLocker => custom_count + 1,
        Screen::Market => custom_count + 2,
        Screen::Messages => custom_count + 3,
        Screen::Settings => 0, // Not part of nav order; jump back to start
    };

    let next_index = (current_index + 1) % total;

    if next_index < custom_count {
        // Switch to Commander with selected custom screen
        let idx = next_index;
        if !layout.custom_screens.is_empty() {
            layout.selected_custom_screen = idx;
            if let Some(sel) = layout.custom_screens.get(layout.selected_custom_screen) {
                if let Some(node) = &sel.layout {
                    layout.overview_panes = Some(crate::settings::build_panes_from_layout(node));
                    layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| crate::settings::layout_leaf_panes(node)));
                } else {
                    layout.overview_panes = None;
                    layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| pane::Type::default_enabled_vec()));
                }
            }

            let _ = crate::settings::Settings::save_from_state(&layout);
            Some(Screen::Commander)
        } else {
            // No custom screens; skip to first fixed tab
            Some(Screen::Materials)
        }
    } else {
        // Fixed tabs
        let fixed_idx = next_index - custom_count;
        match fixed_idx {
            0 => Screen::Materials,
            1 => Screen::ShipLocker,
            2 => Screen::Market,
            _ => Screen::Messages,
        }.into()
    }
}