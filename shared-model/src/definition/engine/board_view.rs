use crate::definition::engine::{Area, PieceRich, TileRich, tile};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoardView {
    pub rows: Vec<BoardRow>,
    pub tiles: Vec<TileRich>,
    pub pieces: Vec<PieceRich>,
    pub special_tile_pairs: Vec<tile::SpecialPair>,
    pub promotion_area: Area,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoardRow(pub Vec<TileRich>);
