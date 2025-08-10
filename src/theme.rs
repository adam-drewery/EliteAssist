use crate::state::State;
use iced::theme::Palette;
use iced::{Color, Theme};

pub const ORANGE: Color = Color::from_rgb(1.0, 0.5, 0.0);

pub const YELLOW: Color = Color::from_rgb(1.0, 1.0, 0.0);

pub const BLUE: Color = Color::from_rgb(0.7, 0.8, 1.0);

pub const GRAY: Color = Color::from_rgb(0.3, 0.3, 0.3);

pub const DARK_GRAY: Color = Color::from_rgb(0.15, 0.15, 0.15);

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

pub mod style {
    use crate::theme::{DARK_GRAY, ORANGE, WHITE};
    use iced::widget::{container};
    use iced::Background::Color;
    use iced::{Border, Theme};
    use iced::border::radius;

    pub fn header(_theme: &Theme) -> container::Style {
        container::Style {
            background: Some(Color(ORANGE)),
            text_color: Some(WHITE),
            border: Default::default(),
            shadow: Default::default(),
        }
    }

    pub fn list_item(_theme: &Theme) -> container::Style {
        container::Style {
            background: Some(DARK_GRAY.into()),
            text_color: Some(WHITE),
            border: Border {
                width: 0.0,
                color: ORANGE,
                radius: radius(0),
            },
            shadow: Default::default(),
        }
    }

    pub fn bordered(_theme: &Theme) -> container::Style {
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

    pub fn progress_bar(_theme: &Theme) -> iced::widget::progress_bar::Style {
        iced::widget::progress_bar::Style {
            background: DARK_GRAY.into(),
            bar: ORANGE.into(),
            border: Border {
                width: 0.5,
                color: ORANGE,
                radius: radius(0),
            },
        }
    }
}