use iced::widget::pane_grid;
use iced::widget::pane_grid::{DragEvent, ResizeEvent};
use serde::{Deserialize, Serialize};
use crate::config::Settings;
use crate::state;

mod navigation;
mod missions;
mod ship;
mod modules;
mod materials;
mod ship_locker;
mod market;
mod journal;
mod loadout;
mod ranks;
mod messages;
mod claims;

pub use navigation::*;
pub use missions::*;
pub use ship::*;
pub use modules::*;
pub use materials::*;
pub use claims::*;
pub use messages::*;
pub use ship_locker::*;
pub use market::*;
pub use journal::*;
pub use loadout::*;
pub use ranks::*;

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
    Materials,
    ShipLocker,
    Market,
    LogJournal,
}

impl Type {

    pub const fn all() -> [Type; 13] {
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
            Type::Materials,
            Type::ShipLocker,
            Type::Market,
            Type::LogJournal,
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
            Type::Materials => "Materials",
            Type::ShipLocker => "Ship Locker",
            Type::Market => "Market",
            Type::LogJournal => "Journal",
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
        layout.current_visible_vec().contains(self)
    }

    pub fn toggle(self, layout: &mut state::Layout, enabled: bool) {

        // Start from the current visible set
        let mut list: Vec<Type> = layout.current_visible_vec();

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

        layout.set_current_visible_vec(list.clone());

        // Mutate the current layout instead of rebuilding to preserve existing splits
        if let Some(panes) = &mut layout.current_panes {
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
    // Initialize with no live panes; default layout is handled by screen::defaults
    layout.current_panes = None;

    // Persist the initialized layout so a settings file exists even before any manual changes
    let _ = Settings::save_from_state(layout);
}

pub fn dragged(layout: &mut state::Layout, event: DragEvent) {
    if let Some(panes) = &mut layout.current_panes {
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

pub(crate) fn resized(layout: &mut state::Layout, event: ResizeEvent) {
    if let Some(panes) = &mut layout.current_panes {
        panes.resize(event.split, event.ratio);

        // Sync the layout into the selected custom screen before saving
        layout.sync_selected_custom_screen_from_live();
        let _ = Settings::save_from_state(&layout);
    }
}