use crate::gui::components::{details, sub_header};
use crate::gui::Message;
use crate::state::State;
use iced::widget::column;

pub struct LoadoutPane;

impl crate::gui::pane::PaneType for LoadoutPane {
    fn id(&self) -> &'static str {
        "loadout"
    }
    fn title(&self) -> &'static str {
        "Loadout"
    }
    fn render<'a>(&self, state: &'a State) -> iced::Element<'a, Message> {
        iced::widget::column![
            sub_header("Suit"),
            details("Name", state.suit_loadout.suit_name),
            details("Loadout", state.suit_loadout.loadout_name.as_ref()),
            column(
                state
                    .suit_loadout
                    .suit_mods
                    .iter()
                    .map(|mod_name| { details("Modification", *mod_name).into() })
            )
            .padding(8),
            sub_header("Weapons"),
            column(state.suit_loadout.modules.iter().map(|module| {
                column![
                    details(&module.slot_name, module.module_name.as_ref()),
                    column(
                        module
                            .weapon_mods
                            .iter()
                            .map(|mod_name| { details("Modification", *mod_name).into() })
                    )
                    .padding([0, 16])
                ]
                .into()
            }))
            .padding(8)
        ]
        .into()
    }
}
