use crate::gui::components::details;
use crate::gui::{pane, Message};
use crate::state::State;
use crate::theme::style;
use crate::theme::GRAY;
use crate::theme::ORANGE;
use crate::{lookup, state};
use iced::widget::image::Handle;
use iced::widget::{column, image, row, scrollable, text, Row};
use iced::{Element, Fill, Left, Right, Top};
use thousands::Separable;

pub struct ShipDetails;
impl pane::Type for ShipDetails {
    fn title(&self) -> &'static str { "Ship" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        let ship_image_bytes =
            lookup::ship_image_bytes(state.ship_loadout.ship_type.as_ref()).unwrap_or_default();
        let ship_image = Handle::from_bytes(ship_image_bytes);

        column![
            scrollable(column![
                ship_title(&state.ship_loadout),
                row![
                    column![image(ship_image).height(160).width(160)].padding(8),
                    column![
                        details(
                            "Rebuy",
                            format!(
                                "CR {}",
                                state.ship_loadout.rebuy.to_string().separate_with_commas()
                            )
                        ),
                        details(
                            "Cargo Capacity",
                            format!("{} T", state.ship_loadout.cargo_capacity)
                        ),
                        details(
                            "Hull Health",
                            format!("{}%", state.ship_loadout.hull_health * 100.0)
                        ),
                        details(
                            "Fuel Capacity (Main)",
                            format!("{} T", state.ship_loadout.fuel_capacity.main)
                        ),
                        details(
                            "Fuel Capacity (Reserve)",
                            format!("{} T", state.ship_loadout.fuel_capacity.reserve)
                        ),
                        details(
                            "Max Jump Range",
                            format!("{:.2} ly", state.ship_loadout.max_jump_range)
                        ),
                        details(
                            "Unladen Mass",
                            format!("{:.2} T", state.ship_loadout.unladen_mass)
                        ),
                        details(
                            "Hull Value",
                            format!(
                                "CR {}",
                                state
                                    .ship_loadout
                                    .hull_value
                                    .to_string()
                                    .separate_with_commas()
                            )
                        ),
                        details(
                            "Modules Value",
                            format!(
                                "CR {}",
                                state
                                    .ship_loadout
                                    .modules_value
                                    .to_string()
                                    .separate_with_commas()
                            )
                        )
                    ]
                ]
            ])
            .style(style::scrollable)
        ]
        .into()
    }
}

fn ship_title(ship_loadout: &state::ship::Loadout) -> Row<'_, Message> {
    row![
        column![
            text(ship_loadout.ship_name.as_ref())
                .color(ORANGE)
                .size(30)
                .align_x(Left),
            text(ship_loadout.ship_type.as_ref())
                .color(ORANGE)
                .size(16)
                .align_x(Left)
        ],
        column![].width(Fill),
        column![
            text(ship_loadout.ship_ident.as_ref())
                .color(GRAY)
                .size(30)
                .align_x(Right)
        ]
    ]
    .padding([0, 8])
    .height(60)
    .align_y(Top)
}
