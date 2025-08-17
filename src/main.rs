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
mod fdev_ids;
mod font;

mod material_locations;
mod edsm;

#[tokio::main]
async fn main() {

    let mut clog = colog::default_builder();
    clog.filter(None, log::LevelFilter::Info);
    clog.init();

    iced::application("EliteAssist", Gui::update, Gui::view)
        .font(font::eurostile::FONT_BYTES)
        .font(font::eurocaps::FONT_BYTES)
        .default_font(font::eurostile::FONT)
        .subscription(subscription)
        .theme(theme)
        .window_size(Size::new(1920.0, 1080.0))
        .centered()
        .antialiasing(true)
        .run()
        .unwrap();
}
