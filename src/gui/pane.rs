use iced::Element;
use iced::widget::pane_grid;
use iced::widget::pane_grid::{DragEvent, ResizeEvent};
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
use crate::gui::Message;
use crate::state::State;

pub trait PaneType: Send + Sync {
    fn id(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn render<'a>(&self, state: &'a State) -> Element<'a, Message>;
}


pub fn all_ids() -> Vec<&'static str> {
    vec![
        "loadout",
        "messages",
        "route",
        "location",
        "ship_details",
        "ship_modules",
        "ranks",
        "missions",
        "claims",
        "materials",
        "ship_locker",
        "market",
        "log_journal",
    ]
}

pub fn default_enabled_ids() -> Vec<&'static str> {
    vec![
        "loadout",
        "messages",
        "route",
        "location",
        "ship_details",
        "ship_modules",
        "ranks",
    ]
}

pub fn make(id: &str) -> Box<dyn PaneType> {
    match id {
        "loadout" => Box::new(LoadoutPane),
        "messages" => Box::new(MessagesPane),
        "route" => Box::new(RoutePane),
        "location" => Box::new(LocationPane),
        "ship_details" => Box::new(ShipDetailsPane),
        "ship_modules" => Box::new(ShipModulesPane),
        "ranks" => Box::new(RanksPane),
        "missions" => Box::new(MissionsPane),
        "claims" => Box::new(ClaimsPane),
        "materials" => Box::new(MaterialsPane),
        "ship_locker" => Box::new(ShipLockerPane),
        "market" => Box::new(MarketPane),
        "log_journal" => Box::new(LogJournalPane),
        _ => Box::new(LoadoutPane),
    }
}

pub fn is_enabled(id: &str, layout: &state::Layout) -> bool {
    layout.current_visible_vec().iter().any(|p| p.as_ref() == id)
}

pub fn toggle(id: &str, layout: &mut state::Layout, enabled: bool) {
    // Start from the current visible set
    let mut list: Vec<Box<str>> = layout.current_visible_vec();

    let was_enabled = list.iter().any(|p| p.as_ref() == id);
    let before_len = list.len();

    if enabled {
        if !was_enabled {
            list.push(id.into());
        }
    } else {
        // Prevent disabling the last remaining pane
        if was_enabled && list.len() > 1 {
            list.retain(|p| p.as_ref() != id);
        }
    }

    // Keep deterministic order according to all_ids()
    let order = all_ids();
    list.sort_by_key(|p| order.iter().position(|q| q == &p.as_ref()).unwrap_or(usize::MAX));

    let did_enable = enabled && !was_enabled;
    let did_disable = !enabled && was_enabled && list.len() < before_len;

    layout.set_current_visible_vec(list.clone());

    // Mutate the current layout instead of rebuilding to preserve existing splits
    if let Some(panes) = &mut layout.current_panes {
        if did_enable {
            // Insert the newly enabled pane by splitting an existing anchor pane
            if let Some((&anchor, _)) = panes.panes.iter().next() {
                let _ = panes.split(pane_grid::Axis::Horizontal, anchor, make(id));
            }
        } else if did_disable {
            // Close the pane containing this pane, preserving another layout
            if let Some(p) = find_with(panes, id) {
                let _ = panes.close(p);
            }
        }
    }

    // Persist settings after visibility/layout changes
    layout.sync_selected_custom_screen_from_live();
    Settings::save_from_state(layout).unwrap_or_else(|e|
        log::error!("Failed to save settings: {}", e));
}

// Helper: find the Pane that contains the given pane id
pub fn find_with(panes: &pane_grid::State<Box<dyn PaneType>>, target_id: &str) -> Option<pane_grid::Pane> {
    for (pane, content) in &panes.panes {
        if content.id() == target_id {
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