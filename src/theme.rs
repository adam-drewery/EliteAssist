use crate::state::State;
use iced::theme::Palette;
use iced::{Color, Theme};

pub const ORANGE: Color = Color::from_rgb(1.0, 0.5, 0.0);

pub const YELLOW: Color = Color::from_rgb(1.0, 1.0, 0.0);

pub const BLUE: Color = Color::from_rgb(0.7, 0.8, 1.0);

pub const GRAY: Color = Color::from_rgb(0.3, 0.3, 0.3);

pub const BACKGROUND: Color = Color::from_rgb(0.1, 0.1, 0.1);

pub const RED: Color = Color::from_rgb(1.0, 0.0, 0.0);

pub const WHITE: Color = Color::WHITE;

pub const BLACK: Color = Color::from_rgb(0.0, 0.0, 0.0);

pub const PALETTE: Palette = Palette {
    text: WHITE,
    background: BACKGROUND,
    danger: RED,
    primary: ORANGE,
    success: BLUE
};

pub fn theme(_state: &State) -> Theme {
    Theme::custom("Elite".into(), PALETTE)
}

pub mod styles {
    use crate::theme::{BLACK, ORANGE};
    use iced::widget::button::{Status, Style};
    use iced::Background::Color;
    use iced::Theme;

    pub fn header_style(_theme: &Theme, _status: Status) -> Style {
        Style {
            background: Some(Color(ORANGE)),
            text_color: BLACK,
            border: Default::default(),
            shadow: Default::default(),
        }
    }
}