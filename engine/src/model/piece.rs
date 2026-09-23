use crate::{Coordinate, faction};

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
    pub faction: faction::Color,
    pub piece_type: self::Type,
}

pub struct PieceWithCoordinate {
    pub faction: faction::Color,
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
    pub fn from_piece(piece: PieceSimple, coordinate: Coordinate) -> Self {
        Self {
            faction: piece.faction,
            piece_type: piece.piece_type,
            coordinate,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PieceRich {
    pub faction: faction::Color,
    pub owner: Option<faction::Color>,
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
    pub fn from_piece(
        piece: PieceSimple,
        owner: Option<faction::Color>,
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
