use crate::event::JournalEvent;
use crate::image::{ENGINEER_ICON, FIXED, GIMBALLED, TURRET};
use crate::state::{ShipModule, SlotType, State};
use crate::theme::styles::header_style;
use crate::theme::{GRAY, ORANGE, WHITE, YELLOW};
use iced::border::radius;
use iced::widget::image::Handle;
use iced::widget::svg::Handle as SvgHandle;
use iced::widget::{button, column, container, image, row, svg, text, Column, Row};
use iced::{Border, Center, Element, Fill, Left, Right, Theme, Top};

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

    let mut result = iced::widget::column![
        button("SHIP").style(header_style).width(Fill),
        row![
            column![
                text(state.ship_loadout.ship_name.to_uppercase())
                    .color(ORANGE)
                    .size(30)
                    .align_x(Left)
            ],
            column![].width(Fill),
            column![
                text(state.ship_loadout.ship_ident.to_uppercase())
                    .color(GRAY)
                    .size(20)
                    .align_x(Right)
            ]
        ]
        .padding([0, 8])
        .height(60)
        .align_y(Top),
    ]
    .padding(8);

    if !weapons.is_empty() {
        result = result
            .push(row![text("HARDPOINTS").color(GRAY).size(30)])
            .push(column(weapons));
    }

    if !utilities.is_empty() {
        result = result
            .push(row![text("UTILITIES").color(GRAY).size(30)])
            .push(column(utilities));
    }

    if !core_internals.is_empty() {
        result = result
            .push(row![text("CORE INTERNALS").color(GRAY).size(30)])
            .push(column(core_internals));
    }

    if !optional_internals.is_empty() {
        result = result
            .push(row![text("OPTIONAL INTERNALS").color(GRAY).size(30)])
            .push(column(optional_internals));
    }

    result
}

fn module_details(module: &ShipModule, size: u8) -> Row<JournalEvent> {
    
    let mut size_column = column![];
    if size != 0 { size_column = size_column.push(text(size).size(24).color(GRAY)); }
    
    row![
        container(row![
            column![
                row![
                    size_column,
                    column![text(module.rating).size(24).color(ORANGE)],
                    column![text(module.class).size(24).color(ORANGE)].padding([0, 6]),
                    column![text(&module.name).size(24).color(ORANGE)].padding([0, 6]),
                    engineering_levels(&module),
                ],
                row![].height(Fill),
                engineering_details(&module)
            ]
            .padding([0, 6])
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
        if size == 0 {
            iced::widget::column![]
        } else {
            match module.mount.as_str() {
                "Fixed" => iced::widget::column![svg(SvgHandle::from_memory(FIXED))],
                "Gimballed" => iced::widget::column![svg(SvgHandle::from_memory(GIMBALLED))],
                "Turreted" => iced::widget::column![svg(SvgHandle::from_memory(TURRET))],
                _ => iced::widget::column![],
            }
        }
        .padding(4)
        .height(30)
        .width(40)
    ]
    .align_x(Right)
}

fn engineering_levels(module: &ShipModule) -> Column<JournalEvent> {
    if let Some(engineering) = &module.engineering {
        iced::widget::column![
            row((0..engineering.level).map(|_| image(Handle::from_bytes(ENGINEER_ICON)).into()))
                .padding(4)
        ]
    } else {
        iced::widget::column![]
    }
    .padding(4)
    .height(30)
}

fn engineering_details(module: &ShipModule) -> Column<JournalEvent> {
    if let Some(engineering) = &module.engineering {
        iced::widget::column![
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
        iced::widget::column![]
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
