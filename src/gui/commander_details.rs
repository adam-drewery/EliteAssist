use crate::event::JournalEvent;
use crate::image::{ENGINEER_ICON, FIXED, GIMBALLED, TURRET};
use crate::state::{SlotType, State};
use crate::theme::styles::header_style;
use crate::theme::{GRAY, ORANGE, RED, WHITE, YELLOW};
use iced::border::radius;
use iced::widget::image::Handle;
use iced::widget::svg::Handle as SvgHandle;
use iced::widget::{button, column, container, image, row, scrollable, svg, text, Column, Row};
use iced::{Border, Center, Element, Fill, Left, Right, Theme, Top};

pub fn commander_details(state: &State) -> Row<JournalEvent> {
    row![
        legal_status(state),
        location(state),
        scrollable(ship(state)),
    ]
    .align_y(Top)
    .height(Fill)
}

fn legal_status(state: &State) -> Column<JournalEvent> {
    column![
        button("LEGAL").style(header_style).width(Fill),
        row!["Legal State: ", text(&state.crime.legal_state)],
        if state.crime.active_fine {
            text("Active Fine").color(RED).width(Fill)
        } else {
            text("No Active Fine").color(GRAY).width(Fill)
        },
        if state.crime.wanted {
            text("Wanted").color(RED).width(Fill)
        } else {
            text("Not Wanted").color(GRAY).width(Fill)
        },
    ]
    .padding(8)
}

fn ship(state: &State) -> Column<JournalEvent> {
    let mut weapons: Vec<Element<JournalEvent>> = vec![];
    let mut utilities: Vec<Element<JournalEvent>> = vec![];
    //let mut core_internals: Vec<Element<JournalEvent>> = vec![];
    //let mut optional_internals: Vec<Element<JournalEvent>> = vec![];

    for module in state.ship_loadout.modules.iter() {
        
        let engineering_icon = if let Some(engineering) = &module.engineering {
            column![row(
                (0..engineering.level).map(|_| image(Handle::from_bytes(ENGINEER_ICON)).into())
            )]
        } else {
            column![]
        };

        let engineering_details = if let Some(engineering) = &module.engineering {
            column![
                row![
                    column![text(&engineering.blueprint_name).size(14).color(GRAY)],
                    if let Some(experimental) = &engineering.experimental_effect {
                        column![text(experimental).size(14).color(YELLOW)]
                    } else {
                        column![]
                    }
                ]
                .padding([0, 6])
            ]
        } else {
            column![]
        };

        match &module.slot {
            SlotType::Hardpoints { number: _, size } => {
                let mount_icon = if *size == 0 {
                    column![]
                } else {
                    match module.mount.as_str() {
                        "Fixed" => column![svg(SvgHandle::from_memory(FIXED))],
                        "Gimballed" => column![svg(SvgHandle::from_memory(GIMBALLED))],
                        "Turreted" => column![svg(SvgHandle::from_memory(TURRET))],
                        _ => column![]
                    }
                };
                
                let row = row![
                    container(column![
                        row![
                            column![text(size).size(24).color(GRAY)].padding([0, 6]),
                            column![text(module.rating).size(24).color(ORANGE)],
                            column![text(module.class).size(24).color(ORANGE)],
                            column![text(&module.name).size(24).color(ORANGE)].padding([0, 6]),
                            engineering_icon.padding(4).height(30).align_x(Left),
                            column![mount_icon.padding(4).height(30).align_x(Right)].width(Fill)
                        ],
                        engineering_details
                    ])
                    .style(module_style)
                    .height(60)
                    .align_y(Center)
                    .width(Fill)
                    .padding(2)
                ]
                .padding(8)
                .align_y(Center)
                .width(Fill)
                .into();

                if *size == 0 {
                    utilities.push(row);
                } else {
                    weapons.push(row);
                }
            }

            SlotType::CoreInternal(_) => {}

            SlotType::OptionalInternal(_) => {}

            SlotType::Cosmetic(_) => {}

            SlotType::Miscellaneous(_) => {}

            SlotType::Unknown(_) => {}
        }
    }

    column![
        button("SHIP").style(header_style).width(Fill),
        row![
            column![
                text(state.ship_loadout.ship_name.to_uppercase())
                    .color(ORANGE)
                    .size(30)
                    .align_x(Left)
            ],
            column![
                text(state.ship_loadout.ship_ident.to_uppercase())
                    .color(GRAY)
                    .size(20)
                    .align_x(Left)
            ]
        ]
        .padding([0, 8])
        .height(60)
        .align_y(Top),
        row![text("HARDPOINTS").color(GRAY).size(30)],
        column(weapons),
        row![text("UTILITIES").color(GRAY).size(30)],
        column(utilities),
    ]
    .padding(8)
}

fn location(state: &State) -> Column<JournalEvent> {
    column![
        button("LOCATION").style(header_style).width(Fill),
        text(&state.commander_name),
    ]
    .padding(8)
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
