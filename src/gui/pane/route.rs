use crate::gui::components::{empty_placeholder, scroll_list};
use crate::gui::{pane, Message};
use crate::image::FUEL_STAR_PNG;
use crate::state::State;
use crate::theme::{style, RED};
use iced::widget::image::Handle;
use iced::widget::{column, container, image, row, text, Row};
use iced::{Element, Fill};

pub struct Route;

impl pane::Type for Route {
    fn title(&self) -> &'static str { "Route" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        if state.nav_route.len() == 0 {
            return column![empty_placeholder("No current route")].into();
        }

        let mut rows: Vec<Row<Message>> = Vec::new();

        for i in 0..state.nav_route.len() {
            let route_step = &state.nav_route[i];
            let distance = if i == 0 {
                &0f64
            } else {
                let prev_step = &state.nav_route[i - 1];
                &prev_step.distance_to(&route_step)
            };
            let mut icons_column = column![];
            let mut star_type_text = text(route_step.star_class.as_ref());

            if route_step.is_fuel_star() {
                icons_column = icons_column.push(
                    row![
                        image(Handle::from_bytes(FUEL_STAR_PNG))
                            .width(12)
                            .height(12)
                    ]
                    .padding(3),
                );
            } else {
                star_type_text = star_type_text.color(RED);
            }

            rows.push(
                row![
                    container(row![
                        column![text(route_step.star_system.as_ref())],
                        column![].width(Fill),
                        column![star_type_text],
                        icons_column,
                        column![].width(16),
                        column![text(format!("{:.2} ly", distance))]
                    ])
                    .style(style::list_item)
                    .padding(8)
                ]
                .padding(8)
                .width(Fill),
            );
        }

        column![scroll_list(rows)].height(Fill).into()
    }
}
