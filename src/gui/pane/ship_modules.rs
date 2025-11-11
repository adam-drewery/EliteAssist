use crate::gui::{pane, Message};
use crate::image::engineering::ENGINEER_ICON_PNG;
use crate::image::ship_modules::*;
use crate::state::State;
use crate::theme::{style, GRAY, ORANGE, YELLOW};
use crate::{bordered_list_item, scroll_list, state};
use iced::widget::image::Handle;
use iced::widget::{column, container, image, row, scrollable, text, Column, Row};
use iced::{Center, Element, Fill, Right};
use thousands::Separable;

pub struct ShipModules;

impl pane::Type for ShipModules {
    fn title(&self) -> &'static str { "Ship Modules" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        column![scroll_list![
            module_group("Hardpoints", HARDPOINTS_PNG, &state.ship_loadout.hardpoints),
            module_group("Utilities", UTILITIES_PNG, &state.ship_loadout.utilities),
            module_group(
                "Core Internals",
                CORE_INTERNAL_PNG,
                &state.ship_loadout.core_internals
            ),
            module_group(
                "Optional Internals",
                OPTIONAL_INTERNAL_PNG,
                &state.ship_loadout.optional_internals
            )
        ]]
        .into()
    }
}

fn module_group<'a>(
    title: &'static str,
    icon_bytes: &'static [u8],
    modules: &'a Vec<state::ship::Module>,
) -> Column<'a, Message> {
    if modules.is_empty() {
        return column![];
    }

    column![
        module_group_title(title, Handle::from_bytes(icon_bytes)),
        column(modules.iter().map(|module| {
            let size = match module.slot {
                state::ship::SlotType::Hardpoints { size, .. } => size,
                _ => 0,
            };
            module_details(module, size).into()
        }))
    ]
}

fn module_group_title(title: &str, icon: Handle) -> Column<'_, Message> {
    column![row![
        column![image(icon).width(40).height(40)],
        column![].width(12),
        column![text(title).color(GRAY).size(30)],
    ]]
}

fn module_details(module: &state::ship::Module, size: u8) -> Row<'_, Message> {
    let mut size_column = column![];
    if size != 0 {
        size_column = size_column.push(text(size).size(24).color(GRAY));
    }

    bordered_list_item![
        column![
            row![
                size_column,
                column![].width(6),
                column![text(module.rating).size(24).color(ORANGE)],
                column![text(module.class).size(24).color(ORANGE)].padding([0, 6]),
                column![text(module.name.as_ref()).size(24).color(ORANGE)].padding([0, 6]),
                engineering_levels(&module),
            ],
            row![].height(Fill),
            engineering_details(&module),
            module_runtime_details(&module)
        ]
        .width(Fill),
        mount_type_icon(module, size)
    ]
}

fn mount_type_icon(module: &state::ship::Module, size: u8) -> Column<'_, Message> {
    column![
        row![].height(Fill),
        if size == 0 {
            row![]
        } else {
            match module.mount.as_ref() {
                "Fixed" => row![image(Handle::from_bytes(FIXED_PNG))],
                "Gimballed" => row![image(Handle::from_bytes(GIMBALLED_PNG))],
                "Turreted" => row![image(Handle::from_bytes(TURRET_PNG))],
                _ => row![],
            }
        }
        .align_y(Center)
        .padding([2, 6])
        .height(30),
        row![].height(Fill),
    ]
    .width(30)
    .align_x(Right)
}

fn engineering_levels(module: &state::ship::Module) -> Column<'_, Message> {
    if let Some(engineering) = &module.engineering {
        column![
            row((0..engineering.level).map(|_| image(Handle::from_bytes(ENGINEER_ICON_PNG)).into()))
                .padding(4)
        ]
    } else {
        column![]
    }
    .padding(4)
    .height(30)
}

fn engineering_details(module: &state::ship::Module) -> Column<'_, Message> {
    if let Some(engineering) = &module.engineering {
        let modifiers_brief = if engineering.modifiers.is_empty() {
            String::new()
        } else {
            engineering
                .modifiers
                .iter()
                .map(|m| {
                    format!(
                        "{}: {} (orig {}), {}",
                        m.label.as_ref(),
                        m.value,
                        m.original_value,
                        if m.less_is_good == 1 { "-" } else { "+" }
                    )
                })
                .collect::<Vec<String>>()
                .join(" | ")
        };

        column![
            row![
                column![
                    text(engineering.blueprint_name.as_ref())
                        .size(14)
                        .color(ORANGE)
                ],
                column![].width(12),
                if let Some(experimental) = &engineering.experimental_effect {
                    column![text(experimental.as_ref()).size(14).color(YELLOW)]
                } else {
                    column![]
                }
            ],
            row![
                column![
                    text(format!("Engineer: {}", engineering.engineer))
                        .size(12)
                        .color(GRAY)
                ],
                column![].width(12),
                column![
                    text(format!("Quality: {:.2}", engineering.quality))
                        .size(12)
                        .color(GRAY)
                ],
            ],
            if !modifiers_brief.is_empty() {
                row![column![text(modifiers_brief).size(12).color(GRAY)]]
            } else {
                row![]
            }
        ]
    } else {
        column![]
    }
}

fn module_runtime_details(module: &state::ship::Module) -> Column<'_, Message> {
    let mut info = Vec::new();
    info.push(format!("On: {}", if module.on { "Yes" } else { "No" }));
    info.push(format!("Priority: {}", module.priority));
    info.push(format!("Health: {:.0}%", module.health * 100.0));
    if let Some(value) = module.value {
        info.push(format!(
            "Value: CR {}",
            value.to_string().separate_with_commas()
        ));
    }
    if let Some(c) = module.ammo_in_clip {
        info.push(format!("Clip: {}", c));
    }
    if let Some(h) = module.ammo_in_hopper {
        info.push(format!("Hopper: {}", h));
    }

    column![row![column![text(info.join("  |  ")).size(12).color(GRAY)]]]
}
