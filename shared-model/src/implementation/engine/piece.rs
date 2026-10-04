use crate::definition::engine::{Coordinate, FactionId, PieceRich, PieceSimple};

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
        owner: Option<FactionId>,
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
