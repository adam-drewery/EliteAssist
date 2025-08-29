use iced::widget::pane_grid;
use iced::widget::pane_grid::{DragEvent, ResizeEvent};
use serde::{Deserialize, Serialize};
use crate::settings::Settings;
use crate::state;
use crate::state::Layout;

pub mod navigation;
pub mod personal;
pub mod ship;
pub mod modules;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    Loadout,
    Messages,
    Route,
    Location,
    ShipDetails,
    ShipModules,
    Ranks,
    Missions,
    Claims,
}

impl Type {

    pub const fn all() -> [Type; 9] {
        [
            Type::Loadout,
            Type::Messages,
            Type::Route,
            Type::Location,
            Type::ShipDetails,
            Type::ShipModules,
            Type::Ranks,
            Type::Missions,
            Type::Claims,
        ]
    }

    pub fn title(&self) -> &'static str {
        match self {
            Type::Loadout => "Loadout",
            Type::Messages => "Messages",
            Type::Route => "Route",
            Type::Location => "Location",
            Type::ShipDetails => "Ship",
            Type::ShipModules => "Ship Modules",
            Type::Ranks => "Ranks",
            Type::Missions => "Missions",
            Type::Claims => "Claims",
        }
    }

    pub fn default_enabled_vec() -> Vec<Type> {
        vec![
            Type::Loadout,
            Type::Messages,
            Type::Route,
            Type::Location,
            Type::ShipDetails,
            Type::ShipModules,
            Type::Ranks,
        ]
    }

    pub fn is_enabled(&self, layout: &state::Layout) -> bool {
        match &layout.enabled_panes {
            Some(v) => v.contains(self),
            None => Type::default_enabled_vec().contains(self),
        }
    }

    pub fn toggle(self, layout: &mut state::Layout, enabled: bool) {

        // Start from the current enabled set (or all panels by default)
        let mut list: Vec<Type> = layout
            .enabled_panes
            .clone()
            .unwrap_or_else(|| Type::default_enabled_vec());

        let was_enabled = list.contains(&self);
        let before_len = list.len();

        if enabled {
            if !was_enabled {
                list.push(self.clone());
            }
        } else {

            // Prevent disabling the last remaining pane
            if was_enabled && list.len() > 1 {
                list.retain(|p| p != &self);
            }
        }

        // Keep deterministic order according to PanelType::all()
        let order = Type::all();
        list.sort_by_key(|p| order.iter().position(|q| q == p).unwrap_or(usize::MAX));

        let did_enable = enabled && !was_enabled;
        let did_disable = !enabled && was_enabled && list.len() < before_len;

        layout.enabled_panes = Some(list.clone());

        // Mutate the current layout instead of rebuilding to preserve existing splits
        if let Some(panes) = &mut layout.overview_panes {
            if did_enable {

                // Insert the newly enabled pane by splitting an existing anchor pane
                if let Some((&anchor, _)) = panes.panes.iter().next() {
                    let _ = panes.split(pane_grid::Axis::Horizontal, anchor, self.clone());
                }
            } else if did_disable {

                // Close the pane containing this pane, preserving another layout
                if let Some(p) = find_with(panes, &self) {
                    let _ = panes.close(p);
                }
            }
        }

        // Persist settings after visibility/layout changes
        layout.sync_selected_custom_screen_from_live();
        Settings::save_from_state(layout).unwrap_or_else(|e|
            log::error!("Failed to save settings: {}", e));
    }

}

pub fn defaults() -> pane_grid::State<Type> {
    let (mut panes, pane_1) = pane_grid::State::new(Type::Loadout);

    let Some((pane_2, split_1)) = panes.split(pane_grid::Axis::Vertical, pane_1, Type::Route) else { return panes; };
    let Some((pane_3, _split_2)) = panes.split(pane_grid::Axis::Vertical, pane_2, Type::ShipDetails) else { return panes; };

    let Some((_, split_3)) = panes.split(pane_grid::Axis::Horizontal, pane_1, Type::Messages) else { return panes; };
    let Some((_, split_4)) = panes.split(pane_grid::Axis::Horizontal, pane_1, Type::Ranks) else { return panes; };
    let Some((_, split_5)) = panes.split(pane_grid::Axis::Horizontal, pane_2, Type::Location) else { return panes; };
    let Some((_, split_6)) = panes.split(pane_grid::Axis::Horizontal, pane_3, Type::ShipModules) else { return panes; };

    // Set vertical splits so each column takes up 1/3 of the space
    panes.resize(split_1, 1.0f32 / 3.0f32);

    // Set horizontal splits
    panes.resize(split_3, 0.66f32);
    panes.resize(split_4, 0.3f32);
    panes.resize(split_5, 0.6f32);
    panes.resize(split_6, 0.3f32);

    panes
}

// Helper: find the Pane that contains the given PanelType
pub fn find_with(panes: &pane_grid::State<Type>, target: &Type) -> Option<pane_grid::Pane> {

    // The iced::pane_grid::State exposes a `panes` field that can be iterated
    for (pane, content) in &panes.panes {
        if content == target {
            return Some(*pane);
        }
    }
    None
}

pub fn load(layout: &mut state::Layout) {

    // Start from the default layout to preserve the intended split structure
    let mut panes = defaults();

    // If some panels are disabled, close them while keeping the rest of the layout
    if let Some(enabled) = &layout.enabled_panes {
        let enabled_set: std::collections::HashSet<_> = enabled.iter().cloned().collect();

        // Collect panes to close first to avoid borrowing issues
        let to_close: Vec<_> = panes
            .panes
            .iter()
            .filter_map(|(pane, content)| if !enabled_set.contains(content) { Some(*pane) } else { None })
            .collect();
        for p in to_close {
            let _ = panes.close(p);
        }
    }
    layout.overview_panes = Some(panes);

    // Persist the initialized layout so a settings file exists even before any manual changes
    let _ = Settings::save_from_state(layout);
}

pub fn dragged(layout: &mut state::Layout, event: DragEvent) {
    if let Some(panes) = &mut layout.overview_panes {
        match event {
            DragEvent::Canceled { .. } => {}
            DragEvent::Picked { .. } => {}
            DragEvent::Dropped { pane, target } => {
                panes.drop(pane, target);
                // Sync into selected custom screen before saving
                layout.sync_selected_custom_screen_from_live();
                let _ = Settings::save_from_state(layout);
            }
        }
    }
}

pub(crate) fn resized(layout: &mut Layout, event: ResizeEvent) {
    if let Some(panes) = &mut layout.overview_panes {
        panes.resize(event.split, event.ratio);

        // Sync the layout into the selected custom screen before saving
        layout.sync_selected_custom_screen_from_live();
        let _ = Settings::save_from_state(&layout);
    }
}