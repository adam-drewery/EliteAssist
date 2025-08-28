use crate::gui::Message;
use crate::image::engineering::ENGINEER_ICON_PNG;
use crate::image::ship_modules::*;
use crate::state;
use crate::state::State;
use crate::theme::{style, GRAY, ORANGE, YELLOW};
use iced::widget::image::Handle;
use iced::widget::{column, container, image, row, scrollable, text, Column, Row};
use iced::{Center, Fill, Right};

pub fn ship_modules(state: &State) -> Column<'_, Message> {

    column![
        scrollable(
            row![
                column![
                    module_group(
                        "Hardpoints",
                        HARDPOINTS_PNG,
                        &state.ship_loadout.hardpoints),
                    module_group(
                        "Utilities",
                        UTILITIES_PNG,
                        &state.ship_loadout.utilities),
                    module_group(
                        "Core Internals",
                        CORE_INTERNAL_PNG,
                        &state.ship_loadout.core_internals),
                    module_group(
                        "Optional Internals",
                        OPTIONAL_INTERNAL_PNG,
                        &state.ship_loadout.optional_internals)
                ], 
                column![].width(12)
            ]
        )
    ]
        .padding(8)
        .height(Fill)
}

fn module_group<'a>(title: &'a str, icon_bytes: &'static [u8], modules: &'a Vec<state::ShipModule>) -> Column<'a, Message> {
    if modules.is_empty() { return column![]; }

    column![
        module_group_title(title, Handle::from_bytes(icon_bytes)),
        column(
            modules.iter().map(|module| {
                let size = match module.slot {
                    state::SlotType::Hardpoints { size, .. } => size,
                    _ => 0,
                };
                module_details(module, size).into()
            })
        )
    ]
}

fn module_group_title(title: &str, icon: Handle) -> Column<'_, Message> {
    column![
        row![
            column![image(icon).width(40).height(40)],
            column![].width(12),
            column![text(title).color(GRAY).size(30)],
        ]
    ]
}

fn module_details(module: &state::ShipModule, size: u8) -> Row<'_, Message> {

    let mut size_column = column![];
    if size != 0 { size_column = size_column.push(text(size).size(24).color(GRAY)); }

    row![
        container(row![
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
                engineering_details(&module)
            ]
            .width(Fill),
            mount_type_icon(module, size)
        ])
        .style(style::bordered)
        .height(48)
        .padding(0.5)
        .width(Fill),
    ]
        .padding(8)
        .align_y(Center)
}

fn mount_type_icon(module: &state::ShipModule, size: u8) -> Column<'_, Message> {
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

fn engineering_levels(module: &state::ShipModule) -> Column<'_, Message> {
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

fn engineering_details(module: &state::ShipModule) -> Column<'_, Message> {
    if let Some(engineering) = &module.engineering {
        column![
            row![
                column![text(engineering.blueprint_name.as_ref()).size(14).color(ORANGE)],
                column![].width(12),
                if let Some(experimental) = &engineering.experimental_effect {
                    column![text(experimental.as_ref()).size(14).color(YELLOW)]
                } else {
                    column![]
                }
            ]
        ]
    } else {
        column![]
    }
}