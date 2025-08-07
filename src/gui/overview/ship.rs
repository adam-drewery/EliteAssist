use crate::font::eurocaps::FONT;
use crate::gui::components::{details, header};
use crate::gui::Message;
use crate::image::{CORE_INTERNAL_PNG, ENGINEER_ICON_PNG, FIXED_PNG, GIMBALLED_PNG, HARDPOINTS_PNG, OPTIONAL_INTERNAL_PNG, TURRET_PNG, UTILITIES_PNG};
use crate::state::{ShipLoadout, ShipModule, SlotType, State};
use crate::theme::{GRAY, ORANGE, WHITE, YELLOW};
use iced::border::radius;
use iced::widget::image::Handle;
use iced::widget::{column, container, image, row, scrollable, text, Column, Row};
use iced::{Border, Center, Element, Fill, Left, Right, Theme, Top};
use thousands::Separable;

pub fn ship_modules(state: &State) -> Column<Message> {
    let mut hardpoints: Vec<Element<Message>> = vec![];
    let mut utilities: Vec<Element<Message>> = vec![];
    let mut core_internals: Vec<Element<Message>> = vec![];
    let mut optional_internals: Vec<Element<Message>> = vec![];

    for module in state.ship_loadout.modules.iter() {
        match &module.slot {
            SlotType::Hardpoints { number: _, size } => {
                let row = module_details(module, *size).into();

                if *size == 0 {
                    utilities.push(row);
                } else {
                    hardpoints.push(row);
                }
            }

            SlotType::CoreInternal(_) => {
                let row = module_details(module, 0).into();
                core_internals.push(row);
            }

            SlotType::OptionalInternal(_) => {
                let row = module_details(module, 0).into();
                optional_internals.push(row);
            }

            SlotType::Cosmetic(_) => {}

            SlotType::Miscellaneous(_) => {}

            SlotType::Unknown(_) => {}
        }
    }

    let mut modules_column = column![];
    if !hardpoints.is_empty() {
        modules_column = modules_column
            .push(module_group_title("Hardpoints", Handle::from_bytes(HARDPOINTS_PNG)))
            .push(column(hardpoints));
    }

    if !utilities.is_empty() {
        modules_column = modules_column
            .push(module_group_title("Utilities", Handle::from_bytes(UTILITIES_PNG)))
            .push(column(utilities));
    }

    if !core_internals.is_empty() {
        modules_column = modules_column
            .push(module_group_title("Core Internals", Handle::from_bytes(CORE_INTERNAL_PNG)))
            .push(column(core_internals));
    }

    if !optional_internals.is_empty() {
        modules_column = modules_column
            .push(module_group_title("Optional Internals", Handle::from_bytes(OPTIONAL_INTERNAL_PNG)))
            .push(column(optional_internals));
    }
    
    column![
        header("Modules"),
        scrollable(row![modules_column, column![].width(12)])
    ]
    .padding(8)
    .height(Fill)
}

pub fn ship_details(state: &State) -> Column<Message> {
    column![
        header("Ship"),
        ship_title(&state.ship_loadout),
        details("Rebuy", "CR ".to_owned() + &state.ship_loadout.rebuy.to_string().separate_with_commas()),
        details("Cargo Capacity", state.ship_loadout.cargo_capacity.to_string() + " T"),
        details("Hull Health", (state.ship_loadout.hull_health * 100.0).to_string() + "%"),
        details("Fuel Capacity (Main)", state.ship_loadout.fuel_capacity.main.to_string() + " T"),
        details("Fuel Capacity (Reserve)", state.ship_loadout.fuel_capacity.reserve.to_string() + " T"),
        details("Max Jump Range", format!("{:.2} ly", state.ship_loadout.max_jump_range)),
        details("Unladen Mass", format!("{:.2} T", state.ship_loadout.unladen_mass)),
    ]
    .padding(8)
}

fn module_group_title(title: &str, icon: Handle) -> Column<Message> {
    column![row![
        column![image(icon).width(40).height(40)],
        column![].width(12),
        column![text(title).font(FONT).color(GRAY).size(30)],
    ]]
}

fn ship_title(ship_loadout: &ShipLoadout) -> Row<Message> {
    row![
            column![
                text(&ship_loadout.ship_name)
                    .font(FONT)
                    .color(ORANGE)
                    .size(30)
                    .align_x(Left),
                text(&ship_loadout.ship_type)
                    .font(FONT)
                    .color(ORANGE)
                    .size(16)
                    .align_x(Left)
            ],
            column![].width(Fill),
            column![
                text(&ship_loadout.ship_ident)
                    .font(FONT)
                    .color(GRAY)
                    .size(30)
                    .align_x(Right)
            ]
        ]
        .padding([0, 8])
        .height(60)
        .align_y(Top)
}

fn module_details(module: &ShipModule, size: u8) -> Row<Message> {
    
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
                    column![text(&module.name).size(24).color(ORANGE)].padding([0, 6]),
                    engineering_levels(&module),
                ],
                row![].height(Fill),
                engineering_details(&module)
            ]
            .width(Fill),
            mount_type_icon(module, size)
        ])
        .style(module_style)
        .height(48)
        .padding(0.5)
        .width(Fill),
    ]
    .padding(8)
    .align_y(Center)
}

fn mount_type_icon(module: &ShipModule, size: u8) -> Column<Message> {
    column![
        row![].height(Fill),
        if size == 0 {
            row![]
        } else {
            match module.mount.as_str() {
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

fn engineering_levels(module: &ShipModule) -> Column<Message> {
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

fn engineering_details(module: &ShipModule) -> Column<Message> {
    if let Some(engineering) = &module.engineering {
        column![
            row![
                column![text(&engineering.blueprint_name).size(14).color(ORANGE)],
                column![].width(12),
                if let Some(experimental) = &engineering.experimental_effect {
                    column![text(experimental).size(14).color(YELLOW)]
                } else {
                    column![]
                }
            ]
        ]
    } else {
        column![]
    }
}

fn module_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: None,
        text_color: Some(WHITE),
        border: Border {
            width: 1.0,
            color: ORANGE,
            radius: radius(0),
        },
        shadow: Default::default(),
    }
}