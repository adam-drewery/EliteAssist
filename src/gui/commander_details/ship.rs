use crate::event::JournalEvent;
use crate::gui::components::{details, header};
use crate::image::{ENGINEER_ICON, FIXED_PNG, GIMBALLED_PNG, TURRET_PNG};
use crate::state::{ShipLoadout, ShipModule, SlotType, State};
use crate::theme::{GRAY, ORANGE, WHITE, YELLOW};
use iced::border::radius;
use iced::widget::image::Handle;
use iced::widget::{column, container, image, row, scrollable, text, Column, Row};
use iced::{Border, Center, Element, Fill, Left, Right, Theme, Top};
use thousands::Separable;

pub fn ship(state: &State) -> Column<JournalEvent> {
    let mut weapons: Vec<Element<JournalEvent>> = vec![];
    let mut utilities: Vec<Element<JournalEvent>> = vec![];
    let mut core_internals: Vec<Element<JournalEvent>> = vec![];
    let mut optional_internals: Vec<Element<JournalEvent>> = vec![];

    for module in state.ship_loadout.modules.iter() {
        match &module.slot {
            SlotType::Hardpoints { number: _, size } => {
                let row = module_details(module, *size).into();

                if *size == 0 {
                    utilities.push(row);
                } else {
                    weapons.push(row);
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

    if !weapons.is_empty() {
        modules_column = modules_column
            .push(column![text("WEAPONS").color(GRAY).size(30)])
            .push(column(weapons));
    }

    if !utilities.is_empty() {
        modules_column = modules_column
            .push(column![text("UTILITIES").color(GRAY).size(30)])
            .push(column(utilities));
    }

    if !core_internals.is_empty() {
        modules_column = modules_column
            .push(column![text("CORE INTERNALS").color(GRAY).size(30)])
            .push(column(core_internals));
    }

    if !optional_internals.is_empty() {
        modules_column = modules_column
            .push(column![text("OPTIONAL INTERNALS").color(GRAY).size(30)])
            .push(column(optional_internals));
    }
    
    column![
        header("SHIP"),
        ship_title(&state.ship_loadout),
        details("REBUY", "CR ".to_owned() + &state.ship_loadout.rebuy.to_string().separate_with_commas()),
        details("CARGO CAPACITY", state.ship_loadout.cargo_capacity.to_string() + " T"),
        details("HULL HEALTH", (state.ship_loadout.hull_health * 100.0).to_string() + "%"),
        details("FUEL CAPACITY (MAIN)", state.ship_loadout.fuel_capacity.main.to_string() + " T"),
        details("FUEL CAPACITY (RESERVE)", state.ship_loadout.fuel_capacity.reserve.to_string() + " T"),
        details("MAX JUMP RANGE", format!("{:.2} LY", state.ship_loadout.max_jump_range)),
        details("UNLADEN MASS", format!("{:.2} T", state.ship_loadout.unladen_mass)),
        header("MODULES"),
        scrollable(row![modules_column, column![].width(12)])
    ]
    .padding(8)
}

fn ship_title(ship_loadout: &ShipLoadout) -> Row<JournalEvent> {
    row![
            column![
                text(ship_loadout.ship_name.to_uppercase())
                    .color(ORANGE)
                    .size(30)
                    .align_x(Left),
                text(ship_loadout.ship.to_uppercase())
                    .color(ORANGE)
                    .size(16)
                    .align_x(Left)
            ],
            column![].width(Fill),
            column![
                text(ship_loadout.ship_ident.to_uppercase())
                    .color(GRAY)
                    .size(30)
                    .align_x(Right)
            ]
        ]
        .padding([0, 8])
        .height(60)
        .align_y(Top)
}

fn module_details(module: &ShipModule, size: u8) -> Row<JournalEvent> {
    
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
            mount_details(module, size)
        ])
        .style(module_style)
        .height(48)
        .padding(0.5)
        .width(Fill),
    ]
    .padding(8)
    .align_y(Center)
}

fn mount_details(module: &ShipModule, size: u8) -> Column<JournalEvent> {
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

fn engineering_levels(module: &ShipModule) -> Column<JournalEvent> {
    if let Some(engineering) = &module.engineering {
        column![
            row((0..engineering.level).map(|_| image(Handle::from_bytes(ENGINEER_ICON)).into()))
                .padding(4)
        ]
    } else {
        column![]
    }
    .padding(4)
    .height(30)
}

fn engineering_details(module: &ShipModule) -> Column<JournalEvent> {
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