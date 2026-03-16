use crate::engine::{Coordinate, faction};
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
pub struct Piece {
    pub faction: faction::Color,
    pub piece_type: self::Type,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PieceExternal {
    pub faction: faction::Color,
    pub owner: Option<faction::Color>,
    pub piece_type: self::Type,
    pub coordinate: Coordinate,
}

impl From<PieceExternal> for Piece {
    fn from(piece: PieceExternal) -> Piece {
        Piece {
            faction: piece.faction,
            piece_type: piece.piece_type,
        }
    }
}

impl PieceExternal {
    pub fn from_piece(piece: Piece, owner: Option<faction::Color>, coordinate: Coordinate) -> Self {
        Self {
            faction: piece.faction,
            owner,
            piece_type: piece.piece_type,
            coordinate,
        }
    }
}
