use crate::definition::engine::{Coordinate, FactionId};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use Type::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PieceSimple {
    pub faction: FactionId,
    pub piece_type: self::Type,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PieceRich {
    pub faction: FactionId,
    pub owner: Option<FactionId>,
    pub piece_type: self::Type,
    pub coordinate: Coordinate,
}
