use crate::{Coordinate, FactionID};

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
    pub faction: FactionID,
    pub piece_type: self::Type,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PieceWithCoordinate {
    pub faction: FactionID,
    pub piece_type: self::Type,
    pub coordinate: Coordinate,
}

impl From<PieceWithCoordinate> for PieceSimple {
    fn from(piece: PieceWithCoordinate) -> PieceSimple {
        PieceSimple {
            faction: piece.faction,
            piece_type: piece.piece_type,
        }
    }
}

impl PieceWithCoordinate {
    pub fn from_simple(piece: PieceSimple, coordinate: Coordinate) -> Self {
        Self {
            faction: piece.faction,
            piece_type: piece.piece_type,
            coordinate,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PieceRich {
    pub faction: FactionID,
    pub owner: Option<FactionID>,
    pub piece_type: self::Type,
    pub coordinate: Coordinate,
}

impl From<PieceRich> for PieceSimple {
    fn from(piece: PieceRich) -> PieceSimple {
        PieceSimple {
            faction: piece.faction,
            piece_type: piece.piece_type,
        }
    }
}

impl PieceRich {
    pub fn from_simple(
        piece: PieceSimple,
        owner: Option<FactionID>,
        coordinate: Coordinate,
    ) -> Self {
        Self {
            faction: piece.faction,
            owner,
            piece_type: piece.piece_type,
            coordinate,
        }
    }
}
