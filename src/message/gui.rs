use crate::gui::pane;
use crate::gui::screen;
use crate::message::Message;
use crate::state::*;
use iced::widget::pane_grid;
use iced::window;
use iced::Task;
use crate::message::Gui::*;

#[derive(Clone, Debug)]
pub enum Gui {
    NavigateTo(Screen),
    NavigateToCustomScreen(usize),

    // Pane grid interactions on the Overview screen
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),

    // Settings and custom screens
    TogglePane(Box<str>, bool),
    AddCustomScreen,
    RemoveCustomScreen,
    SelectCustomScreen(usize),
    RenameCustomScreen(Box<str>),

    // Global hotkeys
    NextTab,

    // Window controls
    ToggleFullscreen,
    ToggleFullscreenWithId(Option<window::Id>),

    // Journal directory selection
    ChooseJournalDir,
    JournalDirChosen(Option<std::path::PathBuf>),
}

impl Gui {
    pub fn update(self, state: &mut State) -> Task<Message> {
        match self {
            
            NavigateTo(screen) => state.active_screen = screen,

            PaneDragged(event) => pane::dragged(&mut state.layout, event),

            PaneResized(event) => pane::resized(&mut state.layout, event),

            AddCustomScreen => screen::add_custom(&mut state.layout),

            RemoveCustomScreen => screen::remove_custom(&mut state.layout),

            SelectCustomScreen(idx) => screen::select_custom(&mut state.layout, idx),

            RenameCustomScreen(name) => screen::rename_custom(&mut state.layout, name),

            NavigateToCustomScreen(idx) => {
                state.active_screen = screen::navigate_to(&mut state.layout, idx);
            }

            NextTab => {
                if let Some(screen) = screen::next_tab(&mut state.layout, &state.active_screen) {
                    state.active_screen = screen;
                }
            }

            TogglePane(pane_title, enabled) => {
                pane::toggle(pane_title.as_ref(), &mut state.layout, enabled);
            }
            
            ToggleFullscreen => {
                return window::get_latest()
                    .map(|id| Message::Gui(ToggleFullscreenWithId(id)));
            }

            ToggleFullscreenWithId(id_opt) => {
                if let Some(id) = id_opt {
                    let mode = if state.layout.fullscreen {
                        window::Mode::Windowed
                    } else {
                        window::Mode::Fullscreen
                    };
                    state.layout.fullscreen = !state.layout.fullscreen;
                    return window::change_mode(id, mode).map(|_: ()| Message::Empty);
                }
            }

            ChooseJournalDir => {
                use rfd::FileDialog;
                return Task::perform(async move {
                    let selected = tokio::task::spawn_blocking(|| {
                        FileDialog::new()
                            .set_title("Select Elite Dangerous Journal Directory")
                            .pick_folder()
                    })
                    .await
                    .ok()
                    .flatten();
                    Message::Gui(JournalDirChosen(selected))
                }, |m| m);
            }

            JournalDirChosen(opt_path) => {
                if let Some(path) = opt_path {
                    // Validate it contains at least one .log file
                    let valid = std::fs::read_dir(&path)
                        .ok()
                        .and_then(|iter| {
                            for entry in iter.flatten() {
                                let p = entry.path();
                                if p.extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("log")).unwrap_or(false) {
                                    return Some(true);
                                }
                            }
                            Some(false)
                        })
                        .unwrap_or(false);

                    if valid {
                        let _ = crate::config::Settings::save_journal_dir(&path);
                        // Cause subscriptions to refresh next cycle; nothing else needed
                    }
                }
            }
        }

        Task::none()
    }
}
