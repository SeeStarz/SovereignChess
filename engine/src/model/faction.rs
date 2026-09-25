use strum::{EnumIter, FromRepr};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FactionId(pub u32);

pub use ColorDefault::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, FromRepr)]
#[repr(u32)]
pub enum ColorDefault {
    White = 0,
    Pink = 1,
    Slate = 2,
    Red = 3,
    Orange = 4,
    Yellow = 5,
    Green = 6,
    Cyan = 7,
    Navy = 8,
    Ash = 9,
    Violet = 10,
    Black = 11,
}

impl From<ColorDefault> for FactionId {
    fn from(value: ColorDefault) -> Self {
        FactionId(value as u32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumIter, FromRepr)]
#[repr(u32)]
pub enum ColorArena {
    White = 0,
    Pink = 1,
    Red = 3,
    Orange = 4,
    Yellow = 5,
    Green = 6,
    Cyan = 7,
    Navy = 8,
    Violet = 9,
    Black = 11,
}

impl From<ColorArena> for FactionId {
    fn from(value: ColorArena) -> Self {
        FactionId(value as u32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Allegiance {
    Ally,
    Neutral,
    Enemy,
}
