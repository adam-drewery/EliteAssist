use crate::gui::components::*;
use crate::gui::Message;
use crate::image::FUEL_STAR_PNG;
use crate::state::State;
use crate::theme::{styles, GRAY, RED};
use iced::widget::image::Handle;
use iced::widget::{column, container, image, row, scrollable, text, Column};
use iced::Fill;
use thousands::Separable;

pub fn location(state: &State) -> Column<Message> {
    column![
        header("Location"),
        row![
            system(state),
            powerplay(state)
        ],
        factions(state),
    ]
    .padding(8)
}

fn factions(state: &State) -> Column<Message> {

    if state.location.factions.is_empty() { return column![] }

    let mut result = column![sub_header("Factions")];

    // todo: why do i have to clone this, i don't wanna
    for faction in &state.location.factions {
        let states =  {
            let state_names: Vec<String> = faction.active_states.iter().map(|s| s.state.clone()).collect();
            format!("{} | {} | {}", faction.allegiance, faction.government, state_names.join(" | "))
        };

        result = result.push(
            column![
                row![
                    column![text(&faction.name)],
                    column![].width(Fill),
                    column![text(states).color(GRAY)],
                ]
            ]
        );
    }

    result
}

fn powerplay(state: &State) -> Column<Message> {

    if state.location.powerplay_state.is_none() { return column![] }

    column![
        sub_header("Powerplay"),
        details("Controller", state.location.controlling_power.clone().unwrap_or_default()),
        details("Progress", state.location.powerplay_state_control_progress.map(|x| format!("{:.2}%", x * 100.0)).unwrap_or_default()),
        details("State", state.location.powerplay_state.clone().unwrap_or_default()),
        details("Reinforcement", state.location.powerplay_state_reinforcement.map(|x| x.to_string()).unwrap_or_default()),
        details("Undermining", state.location.powerplay_state_undermining.map(|x| x.to_string()).unwrap_or_default())
    ]
}

fn system(state: &State) -> Column<Message> {
    column![
        sub_header("System"),
        details("Government", &state.location.system_government),
        details("Economy", &state.location.system_economy),
        details("Population", state.location.population.to_string().separate_with_commas()),
        details("Security", &state.location.system_security.replace(" Security", "")),
        details("Allegiance", &state.location.system_allegiance),
    ]
}

pub fn route(state: &State) -> Column<Message> {
    let title_column = column![header("Route")].padding(8);

    if state.nav_route.len() == 0 {
        return column![
            title_column,
            empty_text("No current route")
        ]
    }

    let mut items_column = column![].padding(8);

    for i in 0..state.nav_route.len() {
        let route_step = &state.nav_route[i];
        if i != 0 {
            let prev_step = &state.nav_route[i - 1];
            let distance = &prev_step.distance_to(&route_step);

            let mut icons_column = column![];
            let mut star_type_text = text(&route_step.star_class);

            if route_step.is_fuel_star() {
                icons_column = icons_column.push(
                    row![
                        image(Handle::from_bytes(FUEL_STAR_PNG)).width(12).height(12)].padding(3)
                );
            }
            else {
                star_type_text = star_type_text.color(RED);
            }

            items_column = items_column.push(
                row![
                    container(row![
                        column![text(&route_step.star_system)],
                        column![].width(Fill),
                        column![star_type_text],
                        icons_column,
                        column![].width(16),
                        column![text(format!("{:.2} ly", distance))]
                    ])
                    .style(styles::list_item)
                    .padding(8)
                ]
                .padding(8)
                .width(Fill),
            );
        }
    }

    column![title_column, scrollable(items_column)].height(Fill)
}

