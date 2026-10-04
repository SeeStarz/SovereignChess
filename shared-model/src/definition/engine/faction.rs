pub use ColorStandard::*;
use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FactionId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u32)]
pub enum ColorStandard {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Allegiance {
    Ally,
    Neutral,
    Enemy,
}
