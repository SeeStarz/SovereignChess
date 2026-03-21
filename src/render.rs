use glam::Vec2;
use raylib::{
    RaylibThread,
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::{
    engine::export::{Coordinate, LegalMove, faction, tile},
    game::Data,
    geometry::{FPosition, FRect, FSize},
    sprite::{CompositeDraw, PieceSprite},
};

pub trait ToColor {
    fn to_color(self) -> Color;
}

impl ToColor for faction::Color {
    fn to_color(self) -> Color {
        use faction::Color::*;
        match self {
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
