pub use ColorStandard::*;
use num_enum::TryFromPrimitive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FactionId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, TryFromPrimitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u32)]
pub enum ColorStandard {
    White = 0,
    Ash = 1,
    Slate = 2,
    Black = 3,
    Pink = 4,
    Red = 5,
    Orange = 6,
    Yellow = 7,
    Green = 8,
    Navy = 9,
    Cyan = 10,
    Violet = 11,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Allegiance {
    Ally,
    Neutral,
    Enemy,
}
