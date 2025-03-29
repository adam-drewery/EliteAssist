use crate::subscription::*;

mod journal_poller;
mod events;
mod gui;
mod subscription;

#[tokio::main]
async fn main() {

    iced::application("EliteAssist", gui::Gui::update, gui::Gui::view)
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

