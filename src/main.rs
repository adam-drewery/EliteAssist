use crate::gui::Gui;
use crate::subscription::subscription;
use crate::theme::theme;
use iced::Font;

mod control;
mod event;
mod gui;
mod journal_poller;
mod state;
mod subscription;
mod theme;
mod image;
mod text;

const FONT_BYTES: &[u8] = include_bytes!("font/eurostile.ttf");
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
