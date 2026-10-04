use crate::shared::{Coordinate, FactionId, PieceSimple, piece};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PieceWithCoordinate {
    pub faction: FactionId,
    pub piece_type: piece::Type,
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
