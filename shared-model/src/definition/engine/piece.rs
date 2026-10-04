use crate::definition::engine::{Coordinate, FactionId};

pub use Type::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PieceSimple {
    pub faction: FactionId,
    pub piece_type: self::Type,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PieceRich {
    pub faction: FactionId,
    pub owner: Option<FactionId>,
    pub piece_type: self::Type,
    pub coordinate: Coordinate,
}
