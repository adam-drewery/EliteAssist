#![windows_subsystem = "windows"]

use iced::Size;
use crate::gui::Gui;
use crate::subscription::subscription;
use crate::theme::theme;

mod gui;
mod journal;
mod state;
mod subscription;
mod theme;
mod image;
mod font;
mod edsm;
mod ardent;
mod inara;
mod lookup;
mod query;
mod settings;

fn main() {

    let mut clog = colog::default_builder();
    clog.filter(None, log::LevelFilter::Info);
    clog.init();

    // Create a Tokio runtime
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // Load the mappings from sources like INARA etc.
    // todo: Eventually lets put this in a subscription so we can report on progress.
    runtime.block_on(async {
        lookup::load().await;
    });

    // Run the Iced application
    let _ = iced::application("EliteAssist", Gui::update, Gui::view)
        .font(font::bytes::EUROSTILE)
        .font(font::bytes::EURO_CAPS)
        .default_font(font::EURO_CAPS)
        .subscription(subscription)
        .theme(theme)
        .window_size(Size::new(1920.0, 1080.0))
        .centered()
        .antialiasing(true)
        .run();

}
