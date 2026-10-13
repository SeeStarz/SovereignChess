use crate::definition::engine::Coordinate;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CastleSource {
    pub king_coordinate: Coordinate,
    pub rook_coordinate: Coordinate,
}
