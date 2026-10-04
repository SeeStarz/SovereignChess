use raylib::color::Color;
use shared_model::definition::engine::{FactionId, faction};

pub trait ToColor {
    fn to_color(self) -> Color;
}

impl ToColor for FactionId {
    fn to_color(self) -> Color {
        use faction::ColorStandard::*;
        let faction = faction::ColorStandard::try_from(self.0).expect("Unknown FactionID");
        match faction {
            White => Color::WHITE,
            Pink => Color::PINK,
            Slate => Color::SLATEGRAY,
            Red => Color::RED,
            Orange => Color::ORANGE,
            Yellow => Color::YELLOW,
            Green => Color::GREEN,
            Cyan => Color::CYAN,
            Navy => Color::NAVY,
            Ash => Color::GRAY,
            Violet => Color::VIOLET,
            Black => Color::BLACK.brightness(0.2),
        }
    }
}
