use strum::{EnumIter, FromRepr};

pub use Color::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, FromRepr)]
#[repr(usize)]
pub enum Color {
    White,
    Pink,
    Slate,
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Navy,
    Ash,
    Violet,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Allegiance {
    Ally,
    Neutral,
    Enemy,
}
