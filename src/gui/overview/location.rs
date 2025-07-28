use crate::event::JournalEvent;
use crate::fonts::eurocaps::FONT;
use crate::gui::components::{details, header, sub_header};
use crate::image::FUEL_STAR_PNG;
use crate::state::State;
use crate::theme::{styles, GRAY, RED};
use iced::widget::image::Handle;
use iced::widget::{column, container, image, row, scrollable, text, Column};
use iced::Fill;
use thousands::Separable;

pub fn location(state: &State) -> Column<JournalEvent> {
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

fn factions(state: &State) -> Column<JournalEvent> {

    if state.location.factions.is_none() { return column![] }

    let mut result = column![sub_header("Factions")];

    // todo: why do i have to clone this, i don't wanna
    for faction in state.location.factions.clone().unwrap() {
        let states = if let Some(active_states) = &faction.active_states {
            let state_names: Vec<String> = active_states.iter().map(|s| s.state.clone()).collect();
            format!("{} | {} | {}", faction.allegiance, faction.government, state_names.join(" | "))
        } else {
            format!("{} | {}", faction.allegiance, faction.government)
        };

        result = result.push(
            column![
                row![
                    column![text(faction.name)],
                    column![].width(Fill),
                    column![text(states).color(GRAY)],
                ]
            ],
            //     column![text(format!("{:.2}%", faction.my_reputation)).color(GRAY)],
            //     column![].width(12),
            //     column![text(format!("{:.2}%", faction.influence * 100.0)).color(GRAY)],
        );
    }

    result
}

fn powerplay(state: &State) -> Column<JournalEvent> {

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

fn system(state: &State) -> Column<JournalEvent> {
    column![
        sub_header("System"),
        details("Government", &state.location.system_government),
        details("Economy", &state.location.system_economy),
        details("Population", state.location.population.to_string().separate_with_commas()),
        details("Security", &state.location.system_security.replace(" Security", "")),
        details("Allegiance", &state.location.system_allegiance),
    ]
}

pub fn route(state: &State) -> Column<JournalEvent> {
    let title_column = column![header("Route")].padding(8);

    if state.nav_route.len() == 0 {
        return column![
            title_column,
            row![].height(Fill),
            row![
                column![].width(Fill),
                column![text("No current route").font(FONT)],
                column![].width(Fill),
            ],
            row![].height(Fill),
        ]
    }

    let mut items_column = column![].padding(8);

    for i in 0..state.nav_route.len() {
        let route_step = &state.nav_route[i];
        if i != 0 {
            let prev_step = &state.nav_route[i - 1];
            let distance = calculate_distance(&prev_step.star_pos, &route_step.star_pos);

            let mut icons_column = column![];
            let mut star_type_text = text(&route_step.star_class);

            if route_step.is_fuel_star() {
                icons_column = icons_column.push(row![image(Handle::from_bytes(FUEL_STAR_PNG)).width(12).height(12)].padding(3));
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

fn calculate_distance(pos1: &Vec<f64>, pos2: &Vec<f64>) -> f64 {
    let dx = pos2[0] - pos1[0];
    let dy = pos2[1] - pos1[1];
    let dz = pos2[2] - pos1[2];
    f64::sqrt(dx * dx + dy * dy + dz * dz)
}

