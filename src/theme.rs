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

    use crate::theme::{BACKGROUND, DARK_GRAY, GRAY, ORANGE, WHITE};
    use iced::border::radius;
    use iced::Background::Color;
    use iced::{Border, Theme};
    use iced::widget::button::Status;

    fn orange_border() -> Border {
        Border {
            width: 1.0,
            color: ORANGE,
            radius: radius(0)
        }
    }

    pub fn header(_theme: &Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(Color(ORANGE)),
            text_color: Some(WHITE),
            border: Default::default(),
            shadow: Default::default(),
        }
    }

    pub fn list_item(_theme: &Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(DARK_GRAY.into()),
            text_color: Some(WHITE),
            border: Default::default(),
            shadow: Default::default(),
        }
    }

    pub fn selected_button(_theme: &Theme, status: Status) -> iced::widget::button::Style {
        iced::widget::button::Style {
            background: Some(DARK_GRAY.into()),
            text_color: match status {
                Status::Active => ORANGE,
                Status::Hovered => WHITE,
                Status::Pressed => ORANGE,
                Status::Disabled => DARK_GRAY,
            },
            border: orange_border(),
            shadow: Default::default(),
        }
    }

    pub fn button(_theme: &Theme, status: Status) -> iced::widget::button::Style {
        iced::widget::button::Style {
            background: Some(DARK_GRAY.into()),
            text_color: match status {
                Status::Active => GRAY,
                Status::Hovered => WHITE,
                Status::Pressed => ORANGE,
                Status::Disabled => GRAY,
            },
            border: Default::default(),
            shadow: Default::default(),
        }
    }

    pub fn bordered(_theme: &Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: None,
            text_color: Some(WHITE),
            border: orange_border(),
            shadow: Default::default(),
        }
    }

    pub fn tooltip(_theme: &Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(Color(BACKGROUND)),
            text_color: Some(ORANGE),
            border: orange_border(),
            shadow: Default::default(),
        }
    }

    pub fn progress_bar(_theme: &Theme) -> iced::widget::progress_bar::Style {
        iced::widget::progress_bar::Style {
            background: DARK_GRAY.into(),
            bar: ORANGE.into(),
            border: orange_border(),
        }
    }
    pub fn icon_button(_theme: &Theme, _status: iced::widget::svg::Status) -> iced::widget::svg::Style {
        iced::widget::svg::Style {
            color: Some(WHITE)
        }
    }
}