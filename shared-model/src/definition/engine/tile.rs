use crate::definition::engine::{Coordinate, FactionId, PieceRich};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TileRich {
    pub piece: Option<PieceRich>,
    pub special: Option<SpecialPair>,
    pub coordinate: Coordinate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpecialPair {
    pub faction: FactionId,
    pub coordinates: [Coordinate; 2],
}
