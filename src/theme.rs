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
    use iced::{Background, Border, Theme};
    use iced::widget;

    fn orange_border() -> Border {
        Border {
            width: 1.0,
            color: ORANGE,
            radius: radius(0)
        }
    }

    pub fn header(_theme: &Theme) -> widget::container::Style {
        widget::container::Style {
            background: Some(Background::Color(ORANGE)),
            text_color: Some(WHITE),
            border: Default::default(),
            shadow: Default::default(),
        }
    }

    pub fn list_item(_theme: &Theme) -> widget::container::Style {
        widget::container::Style {
            background: Some(DARK_GRAY.into()),
            text_color: Some(WHITE),
            border: Default::default(),
            shadow: Default::default(),
        }
    }

    pub fn selected_button(_theme: &Theme, status: widget::button::Status) -> widget::button::Style {
        widget::button::Style {
            background: Some(DARK_GRAY.into()),
            text_color: match status {
                widget::button::Status::Active => ORANGE,
                widget::button::Status::Hovered => WHITE,
                widget::button::Status::Pressed => ORANGE,
                widget::button::Status::Disabled => DARK_GRAY,
            },
            border: orange_border(),
            shadow: Default::default(),
        }
    }

    pub fn button(_theme: &Theme, status: widget::button::Status) -> widget::button::Style {
        widget::button::Style {
            background: Some(DARK_GRAY.into()),
            text_color: match status {
                widget::button::Status::Active => GRAY,
                widget::button::Status::Hovered => WHITE,
                widget::button::Status::Pressed => ORANGE,
                widget::button::Status::Disabled => GRAY,
            },
            border: Default::default(),
            shadow: Default::default(),
        }
    }

    pub fn bordered(_theme: &Theme) -> widget::container::Style {
        widget::container::Style {
            background: None,
            text_color: Some(WHITE),
            border: orange_border(),
            shadow: Default::default(),
        }
    }

    pub fn tooltip(_theme: &Theme) -> widget::container::Style {
        widget::container::Style {
            background: Some(Background::Color(BACKGROUND)),
            text_color: Some(ORANGE),
            border: orange_border(),
            shadow: Default::default(),
        }
    }

    pub fn progress_bar(_theme: &Theme) -> widget::progress_bar::Style {
        widget::progress_bar::Style {
            background: DARK_GRAY.into(),
            bar: ORANGE.into(),
            border: orange_border(),
        }
    }
    pub fn icon_button(_theme: &Theme, _status: widget::svg::Status) -> widget::svg::Style {
        widget::svg::Style {
            color: Some(WHITE)
        }
    }

    pub fn scrollable(_theme: &Theme, status: widget::scrollable::Status) -> widget::scrollable::Style {
        widget::scrollable::Style {
            container: Default::default(),
            vertical_rail: rail(status),
            horizontal_rail: rail(status),
            gap: None,
        }
    }

    fn rail(status: widget::scrollable::Status) -> widget::scrollable::Rail {
        widget::scrollable::Rail {
            background: Background::Color(BACKGROUND).into(),
            border: orange_border(),
            scroller: widget::scrollable::Scroller {
                border: orange_border(),
                color: match status {
                    widget::scrollable::Status::Active => DARK_GRAY.into(),
                    widget::scrollable::Status::Hovered { .. } => ORANGE.into(),
                    widget::scrollable::Status::Dragged { .. } => ORANGE.into(),
                }
            },
        }
    }

    pub fn checkbox(_theme: &Theme, status: widget::checkbox::Status) -> widget::checkbox::Style {
        widget::checkbox::Style {
            background: Background::Color(DARK_GRAY),
            icon_color: ORANGE.into(),
            border: Border {
                color:  match status {
                    widget::checkbox::Status::Active { .. } => GRAY.into(),
                    widget::checkbox::Status::Hovered { .. } =>  ORANGE.into(),
                    widget::checkbox::Status::Disabled { .. } => DARK_GRAY.into(),
                },
                width: 1.0,
                radius: Default::default(),
            },
            text_color: None,
        }
    }
}
