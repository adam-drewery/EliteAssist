use crate::events::{EliteEvent, ShipLocker};
use iced::widget::{Text, column, container, row, scrollable, text};
use iced::{Bottom, Color, Element, Fill, Font, Left, Right};
use thousands::Separable;

pub struct Gui;

const FONT: Font = Font::with_name("Eurostile");
const TITLE_COLOR: Color = Color::from_rgb(1.0, 0.5, 0.0);

#[derive(Default)]
pub struct State {
    commander_name: String,
    credits: String,
    current_system: String,
    current_body: String,
    ship_locker: ShipLocker,
}

impl Gui {
    pub fn view(state: &State) -> Element<EliteEvent> {
        container(column![
            row![
                column![
                    text(&state.commander_name)
                        .size(30)
                        .font(FONT)
                        .color(TITLE_COLOR),
                    text(&state.credits).size(30).font(FONT),
                ]
                .width(Fill)
                .align_x(Left),
                column![
                    text(&state.current_system).size(30).font(FONT),
                    text(&state.current_body).size(30).font(FONT),
                ]
                .width(Fill)
                .align_x(Right),
            ],
            row![
                column![
                    text("ITEMS").size(20).font(FONT).color(TITLE_COLOR),
                    scrollable(column(
                        state
                            .ship_locker
                            .items
                            .clone()
                            .unwrap_or_default()
                            .into_iter()
                            .map(|item| -> Text { text(item.display_name()).size(16).font(FONT) })
                            .map(Element::from)
                    ))
                    .width(Fill)
                ]
                .align_x(Left),
                column![
                    text("COMPONENTS").size(20).font(FONT).color(TITLE_COLOR),
                    scrollable(column(
                        state
                            .ship_locker
                            .components
                            .clone()
                            .unwrap_or_default()
                            .into_iter()
                            .map(|item| -> Text { text(item.display_name()).size(16).font(FONT) })
                            .map(Element::from)
                    ))
                    .width(Fill)
                ]
                .align_x(Left),
            ]
            .align_y(Bottom)
            .height(Fill)
        ])
        .padding(10)
        .center_x(Fill)
        .into()
    }

    pub fn update(state: &mut State, message: EliteEvent) {
        match message {
            EliteEvent::FileHeader(_) => {}
            EliteEvent::Commander(commander) => {
                state.commander_name = "CMDR ".to_owned() + &commander.name.to_uppercase();
            }
            EliteEvent::Materials(_) => {}
            EliteEvent::Rank(_) => {}
            EliteEvent::Progress(_) => {}
            EliteEvent::Reputation(_) => {}
            EliteEvent::EngineerProgress(_) => {}
            EliteEvent::SquadronStartup(_) => {}
            EliteEvent::LoadGame(_) => {}
            EliteEvent::Statistics(_) => {}
            EliteEvent::ReceiveText(_) => {}
            EliteEvent::Location(location) => {
                state.current_system = location.star_system;
            }
            EliteEvent::Powerplay(_) => {}
            EliteEvent::Music(_) => {}
            EliteEvent::SuitLoadout(_) => {}
            EliteEvent::Backpack(_) => {}
            EliteEvent::ShipLocker(ship_locker) => {
                state.ship_locker = ship_locker;
            }
            EliteEvent::Missions(_) => {}
            EliteEvent::Shutdown(_) => {}
            EliteEvent::Loadout(_) => {}
            EliteEvent::BuyAmmo(_) => {}
            EliteEvent::RestockVehicle(_) => {}
            EliteEvent::BuyMicroResources(_) => {}
            EliteEvent::Status(status) => {
                state.credits = status.balance.separate_with_commas() + " CR";

                if status.body_name.is_some() {
                    state.current_body = status.body_name.unwrap()
                }
            }
            EliteEvent::Disembark(disembark) => {
                state.current_body = disembark.body;
            }
            EliteEvent::Embark(embark) => {
                state.current_body = embark.body;
            }
            EliteEvent::NpcCrewPaidWage(_) => {}
            EliteEvent::Cargo(_) => {}
            EliteEvent::Market(_) => {}
            EliteEvent::None => {}
        }
    }
}
