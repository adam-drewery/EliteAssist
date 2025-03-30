use crate::subscription::*;
use crate::theme::theme;
use gui::*;
use iced::Font;

mod journal_poller;
mod event;
mod gui;
mod subscription;
mod theme;
mod controls;
mod state;
mod images;
mod elite_event;

const FONT_BYTES: &[u8] = include_bytes!("fonts/eurostile.ttf");
pub const FONT: Font = Font::with_name("Eurostile");

#[tokio::main]
async fn main() {

    iced::application("EliteAssist", Gui::update, Gui::view)
        .font(FONT_BYTES)
        .default_font(FONT)
        .subscription(subscription)
        .theme(theme)
        .run()
        .unwrap();
}