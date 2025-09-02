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

    // Data source selection
    ChooseDataSource(DataSource),

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
}

impl Gui {
    pub fn update(self, state: &mut State) -> Task<Message> {
        match self {
            
            NavigateTo(screen) => state.active_screen = screen,

            ChooseDataSource(source) => {
                state.data_source = source.clone();
                match source {
                    DataSource::Local => { /* No async work; journal history loader will start via subscriptions */ }
                    DataSource::Capi => {
                        if state.capi_enabled {
                            // Already authenticated; nothing else to do
                        } else {
                            // Kick off OAuth flow
                            let start = Task::perform(async { Message::AuthStarted }, |m| m);
                            let task = Task::perform(async move {
                                match crate::capi::start_oauth_flow().await {
                                    Ok(msg) => msg,
                                    Err(e) => Message::AuthFailed(format!("{}", e).into()),
                                }
                            }, |m| m);
                            return Task::batch(vec![start, task]);
                        }
                    }
                    DataSource::Unselected => {}
                }
            }

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
        }

        Task::none()
    }
}
