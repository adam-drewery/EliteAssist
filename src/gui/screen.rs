mod custom;
mod settings;

use log::error;
pub use custom::*;
pub use settings::*;
use crate::gui::pane;
use crate::state::{Layout, Screen};

fn update_layout_from_custom_screen(layout: &mut Layout, sel: &crate::config::CustomScreen) {
    if let Some(node) = &sel.layout {
        layout.overview_panes = Some(crate::config::build_panes_from_layout(node));
        layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| crate::config::layout_leaf_panes(node)));
    } else {
        layout.overview_panes = None;
        layout.enabled_panes = Some(sel.visible.clone().unwrap_or_else(|| pane::Type::default_enabled_vec()));
    }
}

pub fn add_custom(layout: &mut Layout) {
    let (layout_opt, visible_opt) = if let Some(panes) = &layout.overview_panes {
        let layout_node = crate::config::state_to_node(panes);
        let visible = layout.enabled_panes
            .clone()
            .unwrap_or_else(|| crate::config::layout_leaf_panes(&layout_node));
        (Some(layout_node), Some(visible))
    } else {
        (None, Some(layout.enabled_panes.clone().unwrap_or_else(|| pane::Type::default_enabled_vec())))
    };

    let name = format!("Screen {}", layout.custom_screens.len() + 1).into();
    layout.custom_screens.push(crate::config::CustomScreen {
        name,
        layout: layout_opt,
        visible: visible_opt,
    });
    layout.selected_custom_screen = layout.custom_screens.len().saturating_sub(1);
    crate::config::Settings::save_from_state(&layout)
        .unwrap_or_else(|_| error!("Failed to save layout"));
}

pub fn remove_custom(layout: &mut Layout) {
    if layout.custom_screens.len() > 1 {
        let idx = layout.selected_custom_screen.min(layout.custom_screens.len() - 1);
        layout.custom_screens.remove(idx);

        // Clamp selection
        if layout.selected_custom_screen >= layout.custom_screens.len() {
            if layout.custom_screens.is_empty() { layout.selected_custom_screen = 0; }
            else { layout.selected_custom_screen = layout.custom_screens.len() - 1; }
        }

        // Load the selected screen configuration
        let custom_screen = layout.custom_screens.get(layout.selected_custom_screen).cloned();
        if let Some(sel) = custom_screen {
            update_layout_from_custom_screen(layout, &sel);
        }

        crate::config::Settings::save_from_state(&layout)
            .unwrap_or_else(|_| error!("Failed to save layout"));
    }
}

pub fn select_custom(layout: &mut Layout, idx: usize) {
    if !layout.custom_screens.is_empty() {
        layout.selected_custom_screen = idx.min(layout.custom_screens.len() - 1);
        let custom_screen = layout.custom_screens.get(layout.selected_custom_screen).cloned();
        if let Some(sel) = custom_screen {
            update_layout_from_custom_screen(layout, &sel);
        }
        crate::config::Settings::save_from_state(&layout)
            .unwrap_or_else(|_| error!("Failed to save layout"));
    }
}

pub fn rename_custom(layout: &mut Layout, name: Box<str>) {
    if let Some(sel) = layout.custom_screens.get_mut(layout.selected_custom_screen) {
        sel.name = name;
        crate::config::Settings::save_from_state(&layout)
            .unwrap_or_else(|_| error!("Failed to save layout"));
    }
}

pub fn navigate_to(layout: &mut Layout, idx: usize) -> Screen {
    if !layout.custom_screens.is_empty() {
        let max_idx = layout.custom_screens.len().saturating_sub(1);
        layout.selected_custom_screen = idx.min(max_idx);
        let custom_screen = layout.custom_screens.get(layout.selected_custom_screen).cloned();
        if let Some(sel) = custom_screen {
            update_layout_from_custom_screen(layout, &sel);
        }
        crate::config::Settings::save_from_state(&layout)
            .unwrap_or_else(|_| error!("Failed to save layout"));
    }

    Screen::Custom
}

pub(crate) fn next_tab(layout: &mut Layout, active_screen: &Screen) -> Option<Screen> {
    let custom_count = layout.custom_screens.len();
    
    if custom_count == 0 { return None; }

    let current_index = match active_screen {
        Screen::Custom => {
            if custom_count == 0 { 0 } else { layout.selected_custom_screen.min(custom_count.saturating_sub(1)) }
        }
        Screen::Settings => 0,
    };

    let next_index = if custom_count == 0 { 0 } else { (current_index + 1) % custom_count };

    if custom_count > 0 {
        let idx = next_index;
        layout.selected_custom_screen = idx;
        if let Some(sel) = layout.custom_screens.get(layout.selected_custom_screen).cloned() {
            update_layout_from_custom_screen(layout, &sel);
        }

        crate::config::Settings::save_from_state(&layout)
            .unwrap_or_else(|_| error!("Failed to save layout"));

        Some(Screen::Custom)
    } else {
        None
    }
}