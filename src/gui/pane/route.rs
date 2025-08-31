use iced::{Element, Fill};
use iced::widget::{image, column, row, text, container, scrollable};
use iced::widget::image::Handle;
use crate::gui::components::empty_placeholder;
use crate::gui::{pane, Message};
use crate::image::FUEL_STAR_PNG;
use crate::state::State;
use crate::theme::{style, RED};

pub struct Route;

impl pane::Type for Route {

    fn title(&self) -> &'static str { "Route" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        
        if state.nav_route.len() == 0 {
            return column![empty_placeholder("No current route")].into();
        }

        let mut items_column = column![];

        for i in 0..state.nav_route.len() {
            let route_step = &state.nav_route[i];
            if i != 0 {
                let prev_step = &state.nav_route[i - 1];
                let distance = &prev_step.distance_to(&route_step);

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

                items_column = items_column.push(row![
                    column![
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
                        .width(Fill)
                    ],
                    column![].width(12) // lil hack to give the scrollbar some space.
                ]);
            }
        }

        column![scrollable(items_column).style(style::scrollable)]
            .height(Fill)
            .into()
    }
}
