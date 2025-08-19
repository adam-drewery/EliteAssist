use iced::Font;

pub const EUROSTILE: Font = Font::with_name("Eurostile");

pub const EURO_CAPS: Font = Font::with_name("Euro Caps");

pub mod bytes {
    
    pub const EUROSTILE: &[u8] = include_bytes!("../font/eurostile.ttf");

    pub const EURO_CAPS: &[u8] = include_bytes!("../font/eurocaps.ttf");
}