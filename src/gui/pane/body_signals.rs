use crate::gui::components::{empty_placeholder, scroll_list};
use crate::gui::pane;
use crate::message::Message;
use crate::state::{ScannedBody, State};
use crate::theme::{style, ORANGE};
use iced::widget::{column, container, row, text, Row};
use iced::{Center, Element, Fill};

pub struct BodySignals;

impl pane::Type for BodySignals {

	fn title(&self) -> &'static str { "Body Signals" }

	fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {

		if !state.fss.bodies.is_empty() {

			column![
				scroll_list(
					state.fss.bodies
						.iter()
						.map(|body| body_details(body.1))
						.collect()
				)
			]
			.into()
			
		} else {
			empty_placeholder("No signals found").into()
		}
	}
}

fn body_details(body: &ScannedBody) -> Row<'_, Message> {
	row![
        container(
			row![
				column![].width(6),
				column![text(body.body_name.to_string()).size(24).color(ORANGE)],
				column![text(body.was_mapped).size(24).color(ORANGE)].padding([0, 6]),
				column![text(body.was_discovered).size(24).color(ORANGE)].padding([0, 6])
            .width(Fill)
        ])
        .style(style::bordered)
        .height(64)
        .padding(0.5)
        .width(Fill),
    ]
		.padding(8)
		.align_y(Center)
}