use crate::subscription::*;

mod journal_poller;
mod events;
mod gui;
mod subscription;

const FONT_BYTES: &[u8] = include_bytes!("eurostile.ttf");

#[tokio::main]
async fn main() {

    iced::application("EliteAssist", gui::Gui::update, gui::Gui::view)
        .font(FONT_BYTES)
        .subscription(subscription)
        .run()
        .unwrap();
}

#[derive(Default)]
struct State {
    commander_name: String,
    credits: String,
    current_system: String,
    current_body: String,
}

