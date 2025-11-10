use iced::{Center, Element, Fill};
use iced::widget::{container, row, text, scrollable, column, Row};
use crate::{centered_column, centered_row, state};
use crate::gui::pane;
use crate::message::Message;
use crate::state::State;
use crate::theme::{style, GRAY, ORANGE, WHITE};

pub struct FssSignals;

impl pane::Type for FssSignals {
	fn title(&self) -> &'static str { "FSS Signals" }

	fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
		if !state.fss.bodies.is_empty() {
		
			centered_column![].into()
			
		} else {
			centered_column![
				centered_row![
					text("No signals found")
				]
			]
				.into()
		}
	}
}