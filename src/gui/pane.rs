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
    fn title(&self) -> &'static str;
    fn render<'a>(&self, state: &'a State) -> Element<'a, Message>;
}

macro_rules! define_panes {
    ($($pane_type:ident),* $(,)?) => {
        // Generate all_ids function
        pub fn all() -> Vec<&'static dyn PaneType> {
            vec![
                $(&$pane_type,)*
            ]
        }

        // Generate pane_from_title function
        pub fn from_title(title: &str) -> Box<dyn PaneType> {
            match title {
                $(
                    t if t == $pane_type.title() => Box::new($pane_type),
                )*
                _ => panic!("Unknown pane title: {}", title),
            }
        }
    };
}

// Define all the panes in one place
define_panes! {
    LoadoutPane,
    MessagesPane,
    RoutePane,
    LocationPane,
    ShipDetailsPane,
    ShipModulesPane,
    RanksPane,
    MissionsPane,
    ClaimsPane,
    MaterialsPane,
    ShipLockerPane,
    MarketPane,
    LogJournalPane,
}

pub fn defaults() -> Vec<&'static dyn PaneType> {
    vec![
        &LoadoutPane,
        &MessagesPane,
        &RoutePane,
        &LocationPane,
        &ShipDetailsPane,
        &ShipModulesPane,
        &RanksPane,
    ]
}

pub fn is_enabled(id: &dyn PaneType, layout: &state::Layout) -> bool {
    layout
        .current_visible_vec()
        .iter()
        .any(|p| p.as_ref().title() == id.title())
}

pub fn toggle(title: &str, layout: &mut state::Layout, enabled: bool) {
    // Start from the current visible set
    let mut list: Vec<Box<dyn PaneType>> = layout.current_visible_vec();

    let was_enabled = list.iter().any(|p| p.as_ref().title() == title);
    let before_len = list.len();

    if enabled {
        if !was_enabled {
            list.push(from_title(title));
        }
    } else {
        // Prevent disabling the last remaining pane
        if was_enabled && list.len() > 1 {
            list.retain(|p| p.as_ref().title() != title);
        }
    }

    // Keep deterministic order according to all_ids()
    let order = all();
    list.sort_by_key(|p| order.iter().position(|q| q.title() == p.as_ref().title()).unwrap_or(usize::MAX));

    let did_enable = enabled && !was_enabled;
    let did_disable = !enabled && was_enabled && list.len() < before_len;

    layout.set_current_visible_vec(list);

    // Mutate the current layout instead of rebuilding to preserve existing splits
    if let Some(panes) = &mut layout.current_panes {
        if did_enable {
            // Insert the newly enabled pane by splitting an existing anchor pane
            if let Some((&anchor, _)) = panes.panes.iter().next() {
                let _ = panes.split(pane_grid::Axis::Horizontal, anchor, from_title(title));
            }
        } else if did_disable {
            // Close the pane containing this pane, preserving another layout
            if let Some(p) = find_with(panes, from_title(title)) {
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
pub fn find_with(panes: &pane_grid::State<Box<dyn PaneType>>, target_id: Box<dyn PaneType>) -> Option<pane_grid::Pane> {
    for (pane, content) in &panes.panes {
        if content.title() == target_id.title() {
            return Some(*pane);
        }
    }
    None
}

pub fn load(layout: &mut state::Layout) {
    layout.current_panes = None;
    Settings::save_from_state(layout)
        .unwrap_or_else(|e| log::error!("Failed to save settings: {}", e));
}

pub fn dragged(layout: &mut state::Layout, event: DragEvent) {
    if let Some(panes) = &mut layout.current_panes {
        match event {
            DragEvent::Canceled { .. } => {}
            DragEvent::Picked { .. } => {}
            DragEvent::Dropped { pane, target } => {
                panes.drop(pane, target);
                // Sync into a selected custom screen before saving
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