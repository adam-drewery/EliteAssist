use crate::subscription::*;
use gui::*;
use iced::Font;

mod journal_poller;
mod events;
mod gui;
mod subscription;
mod color;
mod controls;
mod state;
mod images;

const FONT_BYTES: &[u8] = include_bytes!("eurostile.ttf");
pub const FONT: Font = Font::with_name("Eurostile");

#[tokio::main]
async fn main() {

    iced::application("EliteAssist", Gui::update, Gui::view)
        .font(FONT_BYTES)
        .default_font(FONT)
        .subscription(subscription)
        .run()
        .unwrap();
}