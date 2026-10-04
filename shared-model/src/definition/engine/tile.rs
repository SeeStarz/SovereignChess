use crate::definition::engine::{Coordinate, FactionId, PieceRich};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileRich {
    pub piece: Option<PieceRich>,
    pub special: Option<SpecialPair>,
    pub coordinate: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecialPair {
    pub faction: FactionId,
    pub coordinates: [Coordinate; 2],
}
