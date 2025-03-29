use crate::subscription::*;
use gui::*;

mod journal_poller;
mod events;
mod gui;
mod subscription;

const FONT_BYTES: &[u8] = include_bytes!("eurostile.ttf");

#[tokio::main]
async fn main() {

    iced::application("EliteAssist", Gui::update, Gui::view)
        .font(FONT_BYTES)
        .subscription(subscription)
        .run()
        .unwrap();
}