pub mod eurostile {

    use iced::Font;

    pub const FONT_BYTES: &[u8] = include_bytes!("fonts/eurostile.ttf");

    pub const FONT: Font = Font::with_name("Eurostile");
}

//noinspection SpellCheckingInspection
pub mod eurocaps {

    use iced::Font;

    pub const FONT_BYTES: &[u8] = include_bytes!("fonts/eurocaps.ttf");

    pub const FONT: Font = Font::with_name("Euro Caps");
}