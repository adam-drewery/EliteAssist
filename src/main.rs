#![windows_subsystem = "windows"]

use iced::Size;
use crate::gui::Gui;
use crate::subscription::subscription;
use crate::theme::theme;

mod gui;
mod state;
mod subscription;
mod theme;
mod image;
mod font;
mod edsm;
mod ardent;
mod lookup;
mod query;
mod settings;
mod journal;

fn main() {

    let mut clog = colog::default_builder();
    clog.filter(None, log::LevelFilter::Info);
    clog.init();

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
