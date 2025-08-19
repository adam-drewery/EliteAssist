#![allow(dead_code)]

use iced::Size;
use crate::gui::Gui;
use crate::subscription::subscription;
use crate::theme::theme;

mod gui;
mod event;
mod journal;
mod state;
mod subscription;
mod theme;
mod image;
mod text;
mod font;
mod edsm;
mod ardent;
mod inara;
mod lookup;

#[tokio::main]
async fn main() {

    let mut clog = colog::default_builder();
    clog.filter(None, log::LevelFilter::Info);
    clog.init();

    // load the mappings from sources like INARA etc.
    // Eventually lets put this in a subscription so we can report on progress.
    lookup::load().await;

    let _ = iced::application("EliteAssist", Gui::update, Gui::view)
        .font(font::eurostile::FONT_BYTES)
        .font(font::eurocaps::FONT_BYTES)
        .default_font(font::eurostile::FONT)
        .subscription(subscription)
        .theme(theme)
        .window_size(Size::new(1920.0, 1080.0))
        .centered()
        .antialiasing(true)
        .run();
}
